import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:roundcheckbox/roundcheckbox.dart';

import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/request/viewModel.dart';
import '../environment/manager.dart';

class HeaderWidget extends StatelessWidget {
  final RequestViewModel _viewModel;

  const HeaderWidget(this._viewModel, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
            left: 0,
            right: 0,
            height: 25,
            child: Container(
                height: 25,
                decoration: const BoxDecoration(
                    border: Border(
                        top: BorderSide(color: rippleColor, width: 2),
                        bottom: BorderSide(color: rippleColor, width: 2))),
                child: Row(
                  mainAxisSize: MainAxisSize.max,
                  children: [
                    Expanded(
                        flex: 305,
                        child: Container(
                            decoration: const BoxDecoration(
                                border: Border(
                                    right: BorderSide(
                                        color: rippleColor, width: 2))),
                            alignment: Alignment.centerLeft,
                            padding: const EdgeInsets.only(left: 32, bottom: 3),
                            child: const Text(
                              'Name',
                              style: TextStyle(
                                  fontSize: fontSize - 1,
                                  fontWeight: FontWeight.bold),
                            ))),
                    Expanded(
                        flex: 695,
                        child: Container(
                            alignment: Alignment.centerLeft,
                            padding: const EdgeInsets.only(left: 12, bottom: 3),
                            child: const Text(
                              'Value',
                              style: TextStyle(
                                  fontSize: fontSize - 1,
                                  fontWeight: FontWeight.bold),
                            )))
                  ],
                ))),
        Positioned.fill(top: 25, child: HeaderListWidget(_viewModel))
      ],
    );
  }
}

class HeaderListWidget extends StatelessWidget {
  final RequestViewModel _viewModel;

  const HeaderListWidget(this._viewModel, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var children = <Widget>[];

    _viewModel.currentReq().headers.asMap().forEach((key, value) {
      children.add(HeaderItemWidget(key, _viewModel));
    });

    children.add(SizedBox(
        height: 30,
        child: Row(
          children: [
            Expanded(
                flex: 3,
                child: SizedBox(
                    height: 30,
                    child: TextField(
                        focusNode: _viewModel.nameFocusNode,
                        controller: _viewModel.nameController,
                        autofocus: false,
                        maxLength: 64,
                        style: const TextStyle(
                          fontSize: fontSize - 1,
                        ),
                        decoration: const InputDecoration(
                          hintText: 'Name',
                          counterText: '',
                          contentPadding: EdgeInsets.only(left: 33, top: -12),
                          border: InputBorder.none,
                        )))),
            Expanded(
                flex: 7,
                child: SizedBox(
                    height: 30,
                    child: TextField(
                        focusNode: _viewModel.valueFocusNode,
                        controller: _viewModel.valueController,
                        autofocus: false,
                        maxLength: 64,
                        style: const TextStyle(
                          fontSize: fontSize - 1,
                        ),
                        decoration: const InputDecoration(
                          hintText: 'Value',
                          border: InputBorder.none,
                          counterText: '',
                          contentPadding: EdgeInsets.only(left: 12, top: -12),
                        )))),
          ],
        )));
    return SingleChildScrollView(
        child: Column(
      children: children,
    ));
  }
}

class HeaderItemWidget extends StatelessWidget {
  final int _index;
  final RequestViewModel _viewModel;

  const HeaderItemWidget(this._index, this._viewModel, {Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var header = _viewModel.currentReq().headers[_index];
    List<Widget> children = [
      RoundCheckBox(
          size: 16,
          isRound: false,
          checkedWidget: const Icon(
            Icons.check,
            size: iconSize - 2,
          ),
          uncheckedWidget: null,
          isChecked: header.selected,
          checkedColor: Colors.transparent,
          onTap: (value) {
            if (value != null) {
              _viewModel.setHeaderSelect(_index, value);
            }
          })
    ];
    if (header.nameEdit) {
      var controller = TextEditingController();
      controller.text = header.name;
      controller.selection =
          TextSelection(baseOffset: 0, extentOffset: header.name.length);
      FocusNode focusNode = FocusNode();
      focusNode.addListener(() {
        Log.d('focusNode Listener: ${focusNode.hasFocus}');
        if (!focusNode.hasFocus) {
          _viewModel.editHeaderName(_index, false);
          _viewModel.updateHeader(_index, name: controller.value.text);
          controller.dispose();
        }
      });
      children.add(Expanded(
          flex: 3,
          child: Container(
              decoration: const BoxDecoration(
                  border:
                      Border(right: BorderSide(color: rippleColor, width: 2))),
              height: 30,
              padding: const EdgeInsets.only(left: 3, right: 3, top: 2),
              child: TextField(
                  textAlign: TextAlign.left,
                  focusNode: focusNode,
                  controller: controller,
                  autofocus: true,
                  maxLength: 64,
                  style: const TextStyle(
                    fontSize: fontSize - 1,
                  ),
                  decoration: const InputDecoration(
                    counterText: '',
                    contentPadding: EdgeInsets.only(left: 8, top: 0, bottom: 0),
                    border: OutlineInputBorder(
                        borderSide: BorderSide(width: 0.5),
                        borderRadius: BorderRadius.all(Radius.circular(3))),
                  )))));
    } else {
      children.add(Expanded(
          flex: 3,
          child: GestureDetector(
              onDoubleTap: () {
                _viewModel.editHeaderName(_index, true);
              },
              child: Container(
                  alignment: Alignment.centerLeft,
                  height: 25,
                  decoration: const BoxDecoration(
                      border: Border(
                          right: BorderSide(color: rippleColor, width: 2))),
                  padding: const EdgeInsets.only(left: 11),
                  child: Text(header.name,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(fontSize: fontSize - 1))))));
    }
    if (header.valueEdit) {
      var controller = TextEditingController();
      controller.text = header.value;
      controller.selection =
          TextSelection(baseOffset: 0, extentOffset: header.value.length);
      FocusNode focusNode = FocusNode();
      focusNode.addListener(() {
        Log.d('focusNode Listener: ${focusNode.hasFocus}');
        if (!focusNode.hasFocus) {
          _viewModel.editHeaderValue(_index, false);
          _viewModel.updateHeader(_index, value: controller.value.text);
          controller.dispose();
        }
      });
      children.add(Expanded(
          flex: 7,
          child: Container(
              height: 30,
              padding: const EdgeInsets.only(left: 3, right: 3, top: 2),
              child: TextField(
                  focusNode: focusNode,
                  controller: controller,
                  autofocus: true,
                  maxLength: 64,
                  style: const TextStyle(
                    fontSize: fontSize - 1,
                  ),
                  decoration: const InputDecoration(
                    counterText: '',
                    contentPadding: EdgeInsets.only(left: 8, top: 0, right: 3),
                    border: OutlineInputBorder(
                        borderSide: BorderSide(width: 0.5),
                        borderRadius: BorderRadius.all(Radius.circular(3))),
                  )))));
    } else {
      children.add(Expanded(
          flex: 7,
          child: GestureDetector(
              onDoubleTap: () {
                Log.d("onDoubleTap");
                _viewModel.editHeaderValue(_index, true);
              },
              child: Container(
                  color: Colors.transparent,
                  alignment: Alignment.centerLeft,
                  padding: const EdgeInsets.only(left: 11),
                  child: Text(header.value,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(fontSize: fontSize - 1))))));
    }

    children.add(IconButton(
      splashColor: rippleColor,
      splashRadius: 10,
      icon: const Icon(Icons.remove, size: iconSize + 2),
      onPressed: () async {
        bool? delete = await showDeleteConfirmDialog(
            context, 'Delete one header ?', 'Delete the header ${header.name}');
        if (delete != null && delete) {
          _viewModel.deleteHeader(_index);
        }
      },
    ));
    return Container(
      height: 25,
      padding: const EdgeInsets.only(left: 5),
      child: Row(
        children: children,
      ),
    );
  }
}
