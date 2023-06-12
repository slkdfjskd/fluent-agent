import 'package:fluent/widget/response/responseContent.dart';
import 'package:flutter/material.dart';

import '../../api_bridge.dart';
import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/response/viewModel.dart';

class PrimaryTitleBarWidget extends StatelessWidget {
  final ResponseViewModel viewModel;

  const PrimaryTitleBarWidget({Key? key, required this.viewModel})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var info = viewModel.currentResp()?.code == Code.OK ? 'OK' : 'Error';
    var infoColor =
        viewModel.currentResp()?.code == Code.OK ? Colors.blue : Colors.red;

    var url = viewModel.currentResp()!.url == ""
        ? "empty"
        : viewModel.currentResp()!.url;

    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
                constraints: const BoxConstraints(maxWidth: 280),
                decoration: BoxDecoration(
                    border: Border.all(color: Colors.blue, width: 1),
                    borderRadius: const BorderRadius.all(Radius.circular(3))),
                margin: const EdgeInsets.only(
                    top: 10, bottom: 10, left: 5, right: 5),
                padding:
                    const EdgeInsets.only(top: 6, left: 5, right: 5, bottom: 5),
                height: 30,
                child: Text(
                  url,
                  overflow: TextOverflow.ellipsis,
                  maxLines: 1,
                  style: const TextStyle(
                    fontSize: fontSize,
                    color: blackFontColor,
                  ),
                )),
            Container(
                decoration: BoxDecoration(
                    border: Border.all(color: infoColor, width: 1),
                    borderRadius: const BorderRadius.all(Radius.circular(3))),
                margin: const EdgeInsets.only(
                    top: 10, bottom: 10, left: 5, right: 5),
                padding:
                    const EdgeInsets.only(top: 6, left: 5, right: 5, bottom: 5),
                height: 30,
                // width: double.infinity,
                child: Text(
                  info,
                  maxLines: 1,
                  overflow: TextOverflow.clip,
                  style: TextStyle(
                    fontSize: fontSize,
                    color: infoColor,
                  ),
                ))
          ],
        ),
        Container(
            decoration: const BoxDecoration(
                color: backgroundGreyColor,
                border:
                    Border(bottom: BorderSide(color: Colors.grey, width: 0.5))),
            child: Material(
              color: backgroundGreyColor,
              child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: viewModel.tabBar()!.asMap().entries.map((entry) {
                    var index = entry.key;
                    return Container(
                        alignment: Alignment.center,
                        height: 30,
                        width: 95,
                        child: TextButton(
                          style: TextButton.styleFrom(primary: rippleColor),
                          child: Text(
                            entry.value,
                            style: TextStyle(
                                fontSize: fontSize,
                                color: viewModel.selectIndex() == index
                                    ? Colors.blue
                                    : blackFontColor),
                          ),
                          onPressed: () {
                            Log.d('onPressed request index:$index');
                            viewModel.select(index);
                          },
                        ));
                  }).toList()),
            )),
        Expanded(child: ResponseContentWidget(viewModel: viewModel)),
      ],
    );
  }
}
