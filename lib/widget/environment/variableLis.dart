import 'package:flutter/material.dart';

import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/environment/envVariable.dart';
import '../../model/environment/viewModel.dart';

class VariableListWidget extends StatelessWidget {
  final EnvironmentViewModel viewModel;

  const VariableListWidget({Key? key, required this.viewModel})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    Log.d("VariableListWidget build");
    var current = viewModel.currentManagerEnv();
    if (current == null) {
      return Container();
    }
    return ListView.builder(
        shrinkWrap: true,
        itemCount: viewModel.currentManagerEnv()!.list.length,
        itemBuilder: (ctx, index) =>
            _item(index, viewModel.currentManagerEnv()!.list[index]));
  }

  Widget _item(int index, EnvVariableVO variable) {
    Widget nameWidget;
    Widget valueWidget;

    var bgColor = viewModel.variableSelectIndex == index
        ? selectedColor
        : Colors.transparent;
    var textColor = viewModel.variableSelectIndex == index
        ? whiteFontColor
        : blackFontColor;
    Log.d("index: $index  nameEdit: ${variable.nameEdit}");
    if (variable.nameEdit) {
      var controller = TextEditingController();
      controller.text = variable.dto.name;
      controller.selection =
          TextSelection(baseOffset: 0, extentOffset: variable.dto.name.length);
      FocusNode focusNode = FocusNode();
      focusNode.addListener(() {
        Log.d('focusNode Listener: ${focusNode.hasFocus}');
        if (!focusNode.hasFocus) {
          viewModel.editName(index, false);
          viewModel.updateVariable(index, name: controller.value.text);
          controller.dispose();
          focusNode.dispose();
        }
      });
      nameWidget = Container(
          decoration: const BoxDecoration(
              border: Border(right: BorderSide(color: rippleColor, width: 2))),
          height: 30,
          width: 150,
          padding: const EdgeInsets.only(left: 3, right: 3, top: 2),
          child: TextField(
              textAlign: TextAlign.center,
              focusNode: focusNode,
              controller: controller,
              autofocus: true,
              maxLength: 64,
              style: const TextStyle(
                fontSize: fontSize - 1,
              ),
              decoration: const InputDecoration(
                counterText: '',
                contentPadding: EdgeInsets.only(left: 3, top: 0, bottom: 0),
                border: OutlineInputBorder(
                    borderSide: BorderSide(width: 0.5),
                    borderRadius: BorderRadius.all(Radius.circular(3))),
              )));
    } else {
      if (viewModel.variableSelectIndex == index) {}
      nameWidget = GestureDetector(
          onDoubleTap: () {
            Log.d("GestureDetector onDoubleTapDown");
            viewModel.editName(index, true);
          },
          child: Container(
              height: 30,
              decoration: BoxDecoration(
                  color: bgColor,
                  border: const Border(
                      right: BorderSide(color: rippleColor, width: 2))),
              width: 150,
              alignment: Alignment.center,
              padding: const EdgeInsets.only(left: 5, right: 5),
              child: Text(
                variable.dto.name,
                overflow: TextOverflow.ellipsis,
                maxLines: 1,
                style: TextStyle(fontSize: fontSize - 1, color: textColor),
              )));
    }

    if (variable.valueEdit) {
      var controller = TextEditingController();
      controller.text = variable.dto.value;
      controller.selection =
          TextSelection(baseOffset: 0, extentOffset: variable.dto.value.length);
      FocusNode focusNode = FocusNode();
      focusNode.addListener(() {
        Log.d('focusNode Listener: ${focusNode.hasFocus}');
        if (!focusNode.hasFocus) {
          viewModel.editValue(index, false);
          viewModel.updateVariable(index, value: controller.value.text);
          controller.dispose();
          focusNode.dispose();
        }
      });
      valueWidget = Expanded(
          child: Container(
              height: 30,
              padding: const EdgeInsets.only(left: 3, right: 3, top: 0),
              child: TextField(
                  focusNode: focusNode,
                  controller: controller,
                  autofocus: true,
                  maxLength: 256,
                  style: const TextStyle(
                    fontSize: fontSize - 1,
                  ),
                  decoration: const InputDecoration(
                    counterText: '',
                    contentPadding:
                        EdgeInsets.only(left: 5, top: 0, bottom: 0, right: 5),
                    border: OutlineInputBorder(
                        borderSide: BorderSide(width: 0.5),
                        borderRadius: BorderRadius.all(Radius.circular(3))),
                  ))));
    } else {
      valueWidget = Expanded(
          child: GestureDetector(
              onDoubleTap: () {
                Log.d("GestureDetector onDoubleTapDown");
                viewModel.editValue(index, true);
              },
              child: Container(
                  constraints: const BoxConstraints(minWidth: 200),
                  color: bgColor,
                  height: 30,
                  alignment: Alignment.centerLeft,
                  padding: const EdgeInsets.only(left: 8, right: 8),
                  child: Text(
                    variable.dto.value,
                    overflow: TextOverflow.ellipsis,
                    maxLines: 1,
                    style: TextStyle(
                        fontSize: fontSize - 1, height: 1, color: textColor),
                  ))));
    }

    return GestureDetector(
        onTapDown: (TapDownDetails detail) {
          viewModel.selectVariable(index);
        },
        child: SizedBox(
            width: double.infinity,
            child: Row(
              mainAxisSize: MainAxisSize.max,
              children: [
                nameWidget,
                valueWidget,
              ],
            )));
  }
}
