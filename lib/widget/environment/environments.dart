import 'package:dropdown_button2/dropdown_button2.dart';
import 'package:expandable/expandable.dart';
import 'package:fluent/common/colors.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/environment/viewModel.dart';
import 'manager.dart';

class EnvironmentsWidget extends StatefulWidget {
  const EnvironmentsWidget({Key? key}) : super(key: key);

  @override
  State<EnvironmentsWidget> createState() => _EnvironmentsState();
}

class _EnvironmentsState extends State<EnvironmentsWidget> {
  var controller = ExpandableController(initialExpanded: true);

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<EnvironmentViewModel>(context);
    controller.expanded = viewModel.isOpen;
    var children = [
      Positioned(
          width: 85,
          left: 2,
          child: TextButton(
              style: TextButton.styleFrom(primary: rippleColor),
              child: const Row(
                children: [
                  Padding(
                    padding: EdgeInsets.only(bottom: 4, left: 4),
                    child: Text(
                      "Manage",
                      style:
                          TextStyle(fontSize: fontSize, color: blackFontColor),
                    ),
                  ),
                  Icon(Icons.keyboard_double_arrow_right,
                      size: iconSize, color: blackFontColor),
                ],
              ),
              onPressed: () {
                Log.d("onPressed Manage");
                showEnvironmentDialog(context);
              })),
    ];

    if (viewModel.isSelectedEnv()) {
      children.add(Positioned(
          height: 30,
          right: 0,
          child: DropdownButtonHideUnderline(
              child: DropdownButton2<int>(
            dropdownDecoration:
                BoxDecoration(borderRadius: BorderRadius.circular(4.0)),
            buttonDecoration:
                BoxDecoration(borderRadius: BorderRadius.circular(4.0)),
            itemHeight: 30,
            itemPadding: const EdgeInsets.only(left: 7),
            offset: const Offset(-3, 0),
            dropdownElevation: 3,
            value: viewModel.envSelectIndex,
            icon: const Padding(
                padding: EdgeInsets.only(top: 3, right: 3),
                child: Icon(Icons.expand_more,
                    color: blackFontColor, size: iconSize)),
            style: const TextStyle(fontSize: fontSize, color: blackFontColor),
            onChanged: (int? newValue) {
              viewModel.selectEnv(newValue);
            },
            items: viewModel.envList
                .asMap()
                .entries
                .map((entry) => DropdownMenuItem<int>(
                      value: entry.key,
                      child: Container(
                          constraints: const BoxConstraints(maxWidth: 116),
                          padding: const EdgeInsets.only(left: 7),
                          child: Text(
                            entry.value.envName,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          )),
                    ))
                .toList(),
          ))));
    }

    return ExpandablePanel(
      controller: controller,
      theme: const ExpandableThemeData(hasIcon: false),
      header: TextButton.icon(
        style: TextButton.styleFrom(primary: rippleColor),
        icon: Icon(
          Icons.list_alt_outlined,
          size: iconSize,
          color: viewModel.isOpen ? Colors.blue : blackFontColor,
        ),
        label: Text(
          "Environments",
          style: TextStyle(
              fontSize: fontSize,
              color: viewModel.isOpen ? Colors.blue : blackFontColor),
        ),
        onPressed: () {
          viewModel.revertOpen();
        },
      ),
      expanded: SizedBox(
          width: double.infinity,
          height: 30,
          child: Stack(alignment: Alignment.center, children: children)),
      collapsed: Container(),
    );
  }
}

class EnvItem {
  late int id;
  late String name;

  EnvItem({required this.id, required this.name});

  @override
  bool operator ==(dynamic other) => other != null && id == other.id;

  @override
  int get hashCode => id;
}
