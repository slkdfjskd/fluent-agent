import 'package:fluent/common/sizes.dart';
import 'package:fluent/model/indicator/indicator.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../common/colors.dart';

class MiddleTitle extends StatelessWidget {
  const MiddleTitle({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<IndicatorViewModel>(context);
    var title = viewModel.status +
        (viewModel.method == "" ? "" : " " + viewModel.method);
    var children = [
      Positioned(
          child: Container(
              alignment: Alignment.center,
              margin: const EdgeInsets.only(left: 15, right: 15),
              child: Text(
                title,
                overflow: TextOverflow.clip,
                maxLines: 1,
                style: const TextStyle(
                  fontSize: fontSize - 2,
                  color: blackFontColor,
                  fontWeight: FontWeight.bold,
                ),
              ))),
    ];
    if (viewModel.indicatorVisible) {
      children.add(Positioned(
          bottom: 0,
          left: 0,
          right: 0,
          child: SizedBox(
              height: 2,
              child: LinearProgressIndicator(
                // FIXME 暂时用模糊进度条
                backgroundColor: Colors.grey[200],
                valueColor: const AlwaysStoppedAnimation(Colors.blue),
              ))));
    }
    return Container(
      height: 35,
      child: Stack(children: children),
      decoration: const BoxDecoration(
          color: backgroundGreyColor,
          border: Border(
              left: BorderSide(color: rippleColor, width: 2),
              right: BorderSide(color: rippleColor, width: 2))),
    );
  }
}
