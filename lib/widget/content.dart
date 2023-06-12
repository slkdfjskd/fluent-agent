import 'package:fluent/widget/request/request.dart';
import 'package:fluent/widget/response/response.dart';
import 'package:flutter/material.dart';

import '../common/colors.dart';
import 'navigation/navigation.dart';

class ContentWidget extends StatefulWidget {
  const ContentWidget({Key? key}) : super(key: key);

  @override
  State<ContentWidget> createState() => _ContentState();
}

class _ContentState extends State<ContentWidget> {
  @override
  Widget build(BuildContext context) {
    // TODO: implement build
    return Row(
        mainAxisAlignment: MainAxisAlignment.center,
        mainAxisSize: MainAxisSize.max,
        children: <Widget>[
          Expanded(
            flex: 4,
            child: Container(
              constraints: const BoxConstraints(minWidth: 200),
              decoration: BoxDecoration(
                  border: Border.all(color: rippleColor, width: 2)),
              padding: const EdgeInsets.all(3),
              child: const NavigationWidget(),
            ),
          ),
          Expanded(
            flex: 9,
            child: Container(
              constraints: const BoxConstraints(minWidth: 200),
              decoration: const BoxDecoration(
                  color: backgroundGreyColor,
                  border: Border(
                      bottom: BorderSide(color: rippleColor, width: 2),
                      top: BorderSide(color: rippleColor, width: 2))),
              child: const RequestWidget(),
            ),
          ),
          Expanded(
            flex: 7,
            child: Container(
              constraints: const BoxConstraints(minWidth: 200),
              width: double.infinity,
              decoration: BoxDecoration(
                  border: Border.all(color: rippleColor, width: 2)),
              child: const ResponseWidget(),
            ),
          ),
        ]);
  }
}
