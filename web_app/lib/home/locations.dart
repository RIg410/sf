import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/location_provider.dart';
import '../generated/locations.pb.dart';

class LocationsSection extends StatefulWidget {
  final bool showAddress;
  final bool isMobile;

  const LocationsSection({
    super.key,
    this.showAddress = true,
    this.isMobile = false,
  });

  @override
  State<LocationsSection> createState() => _LocationsSectionState();
}

class _LocationsSectionState extends State<LocationsSection> {
  bool _isDropdownOpen = false;
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlayEntry;

  @override
  void dispose() {
    _removeOverlay();
    super.dispose();
  }

  void _removeOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
    setState(() {
      _isDropdownOpen = false;
    });
  }

  void _toggleDropdown() {
    if (_isDropdownOpen) {
      _removeOverlay();
    } else {
      _showDropdown();
    }
  }

  void _showDropdown() {
    final locationProvider = context.read<LocationProvider>();
    if (locationProvider.locations == null ||
        locationProvider.locations!.isEmpty) {
      return;
    }

    final RenderBox renderBox = context.findRenderObject() as RenderBox;
    final size = renderBox.size;

    double dropdownWidth = size.width;
    if (widget.isMobile) {
      final textPainter = TextPainter(textDirection: TextDirection.ltr);
      double maxWidth = 0;

      for (final location in locationProvider.locations!) {
        textPainter.text = TextSpan(
          text: location.name,
          style: Theme.of(context).textTheme.bodyLarge,
        );
        textPainter.layout();
        final itemWidth = textPainter.width + 80;
        if (itemWidth > maxWidth) {
          maxWidth = itemWidth;
        }
      }

      dropdownWidth = maxWidth.clamp(
        size.width,
        MediaQuery.of(context).size.width * 0.9,
      );
    }

    _overlayEntry = OverlayEntry(
      builder: (context) => Positioned(
        width: dropdownWidth,
        child: CompositedTransformFollower(
          link: _layerLink,
          targetAnchor: Alignment.bottomLeft,
          followerAnchor: Alignment.topLeft,
          offset: const Offset(0, 8),
          child: _LocationDropdown(
            locations: locationProvider.locations!,
            selectedLocation: locationProvider.selectedLocation,
            onLocationSelected: (location) {
              locationProvider.selectLocation(location);
              _removeOverlay();
            },
            isMobile: widget.isMobile,
          ),
        ),
      ),
    );

    Overlay.of(context).insert(_overlayEntry!);
    setState(() {
      _isDropdownOpen = true;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<LocationProvider>(
      builder: (context, locationProvider, child) {
        if (locationProvider.isLoading) {
          return const SizedBox(
            height: 48,
            child: Center(child: CircularProgressIndicator()),
          );
        }

        if (locationProvider.error != null) {
          return Text(
            'Ошибка: ${locationProvider.error}',
            style: TextStyle(color: Theme.of(context).colorScheme.error),
          );
        }

        if (locationProvider.locations?.isEmpty ?? true) {
          return const Text('Локации не найдены');
        }

        final selectedLocation = locationProvider.selectedLocation;
        if (selectedLocation == null) return const SizedBox();

        return CompositedTransformTarget(
          link: _layerLink,
          child: InkWell(
            onTap: _toggleDropdown,
            borderRadius: BorderRadius.circular(8),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              child: widget.isMobile
                  ? _MobileLocationDisplay(
                      location: selectedLocation,
                      isDropdownOpen: _isDropdownOpen,
                    )
                  : _DesktopLocationDisplay(
                      location: selectedLocation,
                      showAddress: widget.showAddress,
                      isDropdownOpen: _isDropdownOpen,
                    ),
            ),
          ),
        );
      },
    );
  }
}

class _MobileLocationDisplay extends StatelessWidget {
  final LocationView location;
  final bool isDropdownOpen;

  const _MobileLocationDisplay({
    required this.location,
    required this.isDropdownOpen,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Stack(
          children: [
            Text(location.name, style: Theme.of(context).textTheme.titleMedium),
          ],
        ),
        const SizedBox(width: 8),
        Icon(
          isDropdownOpen ? Icons.arrow_drop_up : Icons.arrow_drop_down,
          color: Theme.of(context).colorScheme.onSurfaceVariant,
        ),
      ],
    );
  }
}

class _DesktopLocationDisplay extends StatelessWidget {
  final LocationView location;
  final bool showAddress;
  final bool isDropdownOpen;

  const _DesktopLocationDisplay({
    required this.location,
    required this.showAddress,
    required this.isDropdownOpen,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(location.name, style: Theme.of(context).textTheme.titleMedium),
            if (showAddress)
              Text(
                location.address,
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
              ),
          ],
        ),
        const SizedBox(width: 12),
        Icon(
          isDropdownOpen ? Icons.arrow_drop_up : Icons.arrow_drop_down,
          color: Theme.of(context).colorScheme.onSurfaceVariant,
        ),
      ],
    );
  }
}

class _LocationDropdown extends StatelessWidget {
  final List<LocationView> locations;
  final LocationView? selectedLocation;
  final Function(LocationView) onLocationSelected;
  final bool isMobile;

  const _LocationDropdown({
    required this.locations,
    required this.selectedLocation,
    required this.onLocationSelected,
    this.isMobile = false,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      elevation: 8,
      borderRadius: BorderRadius.circular(8),
      child: Container(
        constraints: const BoxConstraints(maxHeight: 300),
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.surface,
          borderRadius: BorderRadius.circular(8),
        ),
        child: ListView.builder(
          padding: const EdgeInsets.all(4),
          shrinkWrap: true,
          itemCount: locations.length,
          itemBuilder: (context, index) {
            final location = locations[index];
            final isSelected = location.id == selectedLocation?.id;

            return _LocationDropdownItem(
              location: location,
              isSelected: isSelected,
              onTap: () => onLocationSelected(location),
              isMobile: isMobile,
            );
          },
        ),
      ),
    );
  }
}

class _LocationDropdownItem extends StatelessWidget {
  final LocationView location;
  final bool isSelected;
  final VoidCallback onTap;
  final bool isMobile;

  const _LocationDropdownItem({
    required this.location,
    required this.isSelected,
    required this.onTap,
    this.isMobile = false,
  });

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(4),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: isSelected ? colorScheme.primaryContainer : null,
          borderRadius: BorderRadius.circular(4),
        ),
        child: isMobile
            ? Text(
                location.name,
                style: TextStyle(
                  fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                  color: isSelected
                      ? colorScheme.onPrimaryContainer
                      : colorScheme.onSurface,
                ),
                overflow: TextOverflow.ellipsis,
                maxLines: 1,
              )
            : Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    location.name,
                    style: TextStyle(
                      fontWeight: isSelected
                          ? FontWeight.bold
                          : FontWeight.normal,
                      color: isSelected
                          ? colorScheme.onPrimaryContainer
                          : colorScheme.onSurface,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    location.address,
                    style: Theme.of(context).textTheme.bodySmall?.copyWith(
                      color: isSelected
                          ? colorScheme.onPrimaryContainer.withOpacity(0.7)
                          : colorScheme.onSurfaceVariant,
                    ),
                  ),
                ],
              ),
      ),
    );
  }
}
