import 'package:fluent/widget/titleBar/middleTitle.dart';
import 'package:flutter/material.dart';

Widget titleBar() {
  return Row(
    children: [
      Expanded(
        flex: 7,
        child: Container(
          height: 35,
          color: Colors.white70,
        ),
      ),
      const Expanded(
        flex: 6,
        child: MiddleTitle(),
      ),
      Expanded(
        flex: 7,
        child: Container(
          height: 35,
          color: Colors.white70,
        ),
      ),
    ],
  );
}
