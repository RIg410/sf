import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class Logo extends StatelessWidget {
  const Logo({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 40,
      child: Stack(
        alignment: Alignment.centerLeft,
        children: [
          Positioned(
            top: -3,
            left: 5,
            child: SvgPicture.asset('crown.svg', height: 16),
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
    );
  }
}
