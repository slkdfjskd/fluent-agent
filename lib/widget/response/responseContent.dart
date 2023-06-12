import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:json_editor/json_editor.dart';

import '../../api_bridge.dart';
import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/response/viewModel.dart';

class ResponseContentWidget extends StatelessWidget {
  final ResponseViewModel viewModel;

  const ResponseContentWidget({Key? key, required this.viewModel})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var infos = <EntryDTO>[];

    var resp = viewModel.currentResp();
    if (resp != null) {
      infos = resp.info.infos;
    }

    List<Widget> children = [
      Container(
        padding: const EdgeInsets.only(left: 10, top: 10),
        color: Colors.white,
        alignment: Alignment.topCenter,
        child: _InfoListWidget(infos),
      ),
      _MessageTabBarWidget(viewModel, true)
    ];

    if (viewModel.currentResp()!.code == Code.OK) {
      children.add(_MessageTabBarWidget(viewModel, false));
    } else {
      children.add(Container(
        padding: const EdgeInsets.only(left: 10, top: 10),
        color: Colors.white,
        alignment: Alignment.center,
        child: SelectableText(viewModel.currentResp()!.msg!),
      ));
    }
    return IndexedStack(
      index: viewModel.selectIndex(),
      children: children,
    );
  }
}

class _MessageTabBarWidget extends StatelessWidget {
  final ResponseViewModel viewModel;
  final bool isRequest;

  const _MessageTabBarWidget(this.viewModel, this.isRequest);

  @override
  Widget build(BuildContext context) {
    var tabBar = isRequest
        ? viewModel.currentResp()!.request.titleBar
        : viewModel.currentResp()!.response!.titleBar;

    return Column(children: [
      Container(
          decoration: const BoxDecoration(
              color: backgroundGreyColor,
              border:
                  Border(bottom: BorderSide(color: Colors.grey, width: 0.3))),
          height: 27,
          child: Material(
            color: backgroundGreyColor,
            child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: tabBar.asMap().entries.map((entry) {
                  return getItem(entry);
                }).toList()),
          )),
      Expanded(child: _MessageContentWidget(viewModel, isRequest))
    ]);
  }

  Widget getItem(MapEntry<int, String> entry) {
    var index = entry.key;
    int selectIndex;
    if (isRequest) {
      selectIndex = viewModel.currentResp()!.request.selectIndex;
    } else {
      selectIndex = viewModel.currentResp()!.response!.selectIndex;
    }
    return Container(
        color: backgroundGreyColor,
        alignment: Alignment.center,
        height: 20,
        width: 95,
        child: TextButton(
          style: TextButton.styleFrom(primary: rippleColor),
          child: Text(
            entry.value,
            style: TextStyle(
                fontSize: fontSize - 1,
                color: selectIndex == index ? Colors.blue : blackFontColor),
          ),
          onPressed: () {
            Log.d('onPressed request index:$index');
            viewModel.selectSecondary(index, isRequest);
          },
        ));
  }
}

class _MessageContentWidget extends StatelessWidget {
  final ResponseViewModel viewModel;

  final bool isRequest;

  const _MessageContentWidget(this.viewModel, this.isRequest);

  @override
  Widget build(BuildContext context) {
    var headers = <EntryDTO>[];
    var body = "";
    if (isRequest) {
      var h = viewModel.currentResp()?.request.headers;
      if (h != null) {
        headers = h;
      }
      var b = viewModel.currentResp()?.request.body;
      if (b != null) {
        body = b;
      }
    } else {
      var h = viewModel.currentResp()?.response?.headers;
      if (h != null) {
        headers = h;
      }
      var b = viewModel.currentResp()?.response?.body;
      if (b != null) {
        body = b;
      }
    }

    var children = [
      Container(
        padding: const EdgeInsets.only(left: 5),
        color: Colors.white,
        alignment: Alignment.center,
        child: _HeaderWidget(headers),
      ),
      Container(
        padding: const EdgeInsets.only(left: 5),
        color: Colors.white,
        alignment: Alignment.center,
        child: jsonEditor(body),
      )
    ];

    return IndexedStack(
      index: viewModel.selectSecondaryIndex(isRequest),
      children: children,
    );
  }

  JsonEditor jsonEditor(String json) {
    return JsonEditor.string(
      enabled: false,
      jsonString: json,
    );
  }
}

class _HeaderWidget extends StatelessWidget {
  final List<EntryDTO> _headers;

  const _HeaderWidget(this._headers);

  @override
  Widget build(BuildContext context) {
    return Stack(
      alignment: AlignmentDirectional.topCenter,
      children: [
        Positioned(
            left: 0,
            right: 0,
            height: 25,
            child: Container(
                height: 25,
                decoration: const BoxDecoration(
                    border: Border(
                        // top: BorderSide(color: rippleColor, width: 2),
                        bottom: BorderSide(color: rippleColor, width: 1))),
                child: Row(
                  mainAxisSize: MainAxisSize.max,
                  children: [
                    Expanded(
                        flex: 3,
                        child: Container(
                            decoration: const BoxDecoration(
                                border: Border(
                                    right: BorderSide(
                                        color: rippleColor, width: 2))),
                            alignment: Alignment.centerLeft,
                            padding: const EdgeInsets.only(left: 8, bottom: 3),
                            child: const Text(
                              'Name',
                              style: TextStyle(
                                  fontSize: fontSize - 1,
                                  fontWeight: FontWeight.bold),
                            ))),
                    Expanded(
                        flex: 7,
                        child: Container(
                            alignment: Alignment.centerLeft,
                            padding: const EdgeInsets.only(left: 8, bottom: 3),
                            child: const Text(
                              'Value',
                              style: TextStyle(
                                  fontSize: fontSize - 1,
                                  fontWeight: FontWeight.bold),
                            )))
                  ],
                ))),
        Positioned.fill(top: 25, child: _HeaderListWidget(_headers))
      ],
    );
  }
}

class _HeaderListWidget extends StatelessWidget {
  final List<EntryDTO> _headers;

  const _HeaderListWidget(this._headers);

  @override
  Widget build(BuildContext context) {
    var children = <Widget>[];
    for (var header in _headers) {
      children.add(_HeaderItemWidget(header));
    }
    return SingleChildScrollView(
        child: Column(
      children: children,
    ));
  }
}

class _HeaderItemWidget extends StatelessWidget {
  final EntryDTO _item;

  const _HeaderItemWidget(this._item);

  @override
  Widget build(BuildContext context) {
    return Container(
        constraints: const BoxConstraints(
          minHeight: 25,
        ),
        decoration: const BoxDecoration(
            border: Border(
          bottom: BorderSide(color: rippleColor, width: 1),
        )),
        child: Row(
          children: [
            Expanded(
                flex: 3,
                child: Container(
                    padding: const EdgeInsets.only(left: 8),
                    decoration: const BoxDecoration(
                        border: Border(
                      right: BorderSide(color: rippleColor, width: 1),
                    )),
                    alignment: Alignment.centerLeft,
                    child: SelectableText(
                      _item.name,
                      style: const TextStyle(fontSize: fontSize - 1),
                    ))),
            Expanded(
                flex: 7,
                child: Container(
                    padding: const EdgeInsets.only(left: 8),
                    alignment: Alignment.centerLeft,
                    child: SelectableText(_item.value,
                        style: const TextStyle(fontSize: fontSize - 1)))),
          ],
        ));
  }
}

class _InfoListWidget extends StatelessWidget {
  final List<EntryDTO> _infos;

  const _InfoListWidget(this._infos);


  @override
  Widget build(BuildContext context) {
    var children = <Widget>[];
    for (var info in _infos) {
      children.add(_InfoItemWidget(info));
    }
    return SingleChildScrollView(
        child: Column(
          children: children,
        ));
  }
  
}

class _InfoItemWidget extends StatelessWidget {
  final EntryDTO _item;

  const _InfoItemWidget(this._item);

  @override
  Widget build(BuildContext context) {
    return Container(
        constraints: const BoxConstraints(
          minHeight: 25,
        ),
        decoration: const BoxDecoration(
            border: Border(
              bottom: BorderSide(color: rippleColor, width: 1),
            )),
        child: Row(
          children: [
            Expanded(
                flex: 5,
                child: Container(
                    padding: const EdgeInsets.only(left: 8),
                    decoration: const BoxDecoration(
                        border: Border(
                          right: BorderSide(color: rippleColor, width: 1),
                        )),
                    alignment: Alignment.centerLeft,
                    child: SelectableText(
                      _item.name,
                      style: const TextStyle(fontSize: fontSize - 1),
                    ))),
            Expanded(
                flex: 5,
                child: Container(
                    padding: const EdgeInsets.only(left: 8),
                    alignment: Alignment.centerLeft,
                    child: SelectableText(_item.value,
                        style: const TextStyle(fontSize: fontSize - 1)))),
          ],
        ));
  }
}
