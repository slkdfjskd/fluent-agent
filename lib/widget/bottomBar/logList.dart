import 'package:expandable/expandable.dart';
import 'package:fluent/common/log.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../api_bridge.dart';
import '../../common/colors.dart';
import '../../common/sizes.dart';
import '../../model/log/viewModel.dart';

class LogListWidget extends StatelessWidget {
  const LogListWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<LogViewModel>(context);
    if (viewModel.isEmpty()) {
      viewModel.loadNextPage();
    }

    var widget = NotificationListener(
        child: ListView.builder(
            controller: viewModel.controller,
            padding:
                const EdgeInsets.only(left: 5, right: 5, top: 5, bottom: 5),
            shrinkWrap: true,
            itemCount: viewModel.length(),
            itemBuilder: (ctx, index) => _LogItemWidget(viewModel.item(index))));
    return widget;
  }
}

class _LogItemWidget extends StatelessWidget {
  final RequestLogVO _item;

  const _LogItemWidget(this._item, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var info = "OK";
    var infoColor = Colors.blue;
    final error = _item.requestLog.error;
    if (error != null) {
      info = "Error";
      infoColor = Colors.red;
    }

    var widget = ExpandableNotifier(
        child: ExpandablePanel(
      controller: _item.controller,
      theme: const ExpandableThemeData(
          iconPadding: EdgeInsets.only(left: 5, right: 0, top: 9, bottom: 6),
          iconRotationAngle: 1.6,
          expandIcon: Icons.navigate_next,
          inkWellBorderRadius: BorderRadius.all(Radius.circular(5)),
          iconPlacement: ExpandablePanelIconPlacement.left,
          iconSize: iconSize),
      header: Container(
          height: 32,
          // decoration: const BoxDecoration(
          //     border: Border(
          //         bottom: BorderSide(width: 0.5, color: rippleColor))),
          alignment: Alignment.centerLeft,
          margin: const EdgeInsets.only(left: 10),
          child: Row(
            children: [
              Expanded(
                  flex: 95,
                  child: Text(
                    "${_item.requestLog.createdAt}   ${_item.requestLog.baseUrl}${_item.requestLog.path}",
                    style: const TextStyle(
                        fontSize: fontSize,
                        overflow: TextOverflow.ellipsis,
                        color: blackFontColor),
                  )),
              Expanded(
                  flex: 5,
                  child: Text(info,
                      style: TextStyle(color: infoColor, fontSize: fontSize)))
            ],
          )),
      collapsed: Container(),
      expanded: _LogItemExpandedWidget(_item),
    ));
    return widget;
  }

}

class _LogItemExpandedWidget extends StatelessWidget {

  final RequestLogVO _item;

  const _LogItemExpandedWidget(this._item, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    List<Widget> respWidgets = [];
    final error = _item.requestLog.error;
    final resp = _item.requestLog.response;
    if (error != null) {
      respWidgets.add(_LogItemEntryWidget(
          "Error:", "code: ${error.code.name}  msg: ${error.msg}"));
    } else if (resp != null) {
      respWidgets.add(
          _LogItemEntryWidget("Response Metadata:", parseHeaders(resp.metadata)));
      respWidgets.add(_LogItemEntryWidget("Response Body:", resp.body));
    }
    return Container(
        margin: const EdgeInsets.only(left: 38, right: 80),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _LogItemEntryWidget("Host:", _item.requestLog.baseUrl),
            _LogItemEntryWidget("Path:", _item.requestLog.path),
            _LogItemEntryWidget("Request Metadata:",
                parseHeaders(_item.requestLog.request.metadata)),
            _LogItemEntryWidget(
                "Request Body:", _item.requestLog.request.body),
            ...respWidgets
          ],
        )
    );
  }

}

class _LogItemEntryWidget extends StatelessWidget {
  final String _name;
  final String _value;

  const _LogItemEntryWidget(this._name, this._value, {Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(top: 4, bottom: 4),
      alignment: Alignment.topLeft,
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Expanded(
              flex: 130,
              child: Text(_name,
                  textAlign: TextAlign.left,
                  style: const TextStyle(fontSize: fontSize))),
          Expanded(
              flex: 889,
              child: SelectableText(_value,
                  style:
                      const TextStyle(fontSize: fontSize, color: Colors.blue)))
        ],
      ),
    );
  }
}

String parseHeaders(List<EntryDTO> headers) {
  var result = "";
  for (var header in headers) {
    if (result.isNotEmpty) {
      result += "\n";
    }
    result += "${header.name}: ${header.value}";
  }
  return result;
}
