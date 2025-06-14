import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';
import 'locations.dart';

class LogoWithLocation extends StatelessWidget {
  const LogoWithLocation({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        SizedBox(
          height: 40,
          child: Stack(
            alignment: Alignment.centerLeft,
            children: [
              Positioned(
                top: 3,
                left: 11,
                child: SvgPicture.asset('crown.svg', height: 10),
              ),
              const Padding(
                padding: EdgeInsets.only(left: 15),
                child: Text(
                  'SoulFamily',
                  style: TextStyle(fontWeight: FontWeight.w200),
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 4),
        const LocationsSection(showAddress: true, isMobile: false),
      ],
    );
  }
}