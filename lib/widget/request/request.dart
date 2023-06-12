import 'package:fluent/common/colors.dart';
import 'package:fluent/common/log.dart';
import 'package:flutter/material.dart';
import 'package:json_editor/json_editor.dart';
import 'package:provider/provider.dart';
import '../../common/sizes.dart';
import '../../model/request/viewModel.dart';
import 'header.dart';

class RequestWidget extends StatefulWidget {
  const RequestWidget({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => RequestState();
}

class RequestState extends State<RequestWidget>
    with SingleTickerProviderStateMixin {
  late TextEditingController textFieldController;
  // FocusNode focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    textFieldController =
        TextEditingController.fromValue(const TextEditingValue());
  }

  @override
  void dispose() {
    textFieldController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<RequestViewModel>(context);
    textFieldController.text = viewModel.currentReq().url;
    textFieldController.addListener(() {
      Log.d("textFieldController listener ${textFieldController.text}");
      viewModel.updateRequestUrl(textFieldController.text);
    });
    return Column(
      children: [
        Container(
            padding:
                const EdgeInsets.only(left: 5, right: 5, top: 10, bottom: 10),
            height: 50,
            child: Material(
                color: backgroundGreyColor,
                child: TextField(
                  controller: textFieldController,
                  autofocus: false,
                  maxLength: 256,
                  style: const TextStyle(
                    fontSize: fontSize,
                  ),
                  decoration: InputDecoration(
                    counterText: '',
                    contentPadding: const EdgeInsets.only(top: 0, bottom: 0),
                    border: const OutlineInputBorder(
                        borderSide: BorderSide(width: 0.5),
                        borderRadius: BorderRadius.all(Radius.circular(3))),
                    hintText: 'Enter request host',
                    prefixIcon: const Icon(Icons.domain, size: iconSize + 2),
                    suffixIcon: IconButton(
                      splashRadius: 13,
                      icon: const Icon(Icons.refresh, size: iconSize + 2),
                      onPressed: () async {
                        Log.d('refresh onPressed');
                        await viewModel.sendRequest();
                      },
                    ),
                  ),
                ))),
        Container(
            decoration: const BoxDecoration(
                color: backgroundGreyColor,
                border:
                    Border(bottom: BorderSide(color: Colors.grey, width: 0.5))),
            child: Material(
              color: backgroundGreyColor,
              child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: viewModel.tabBar().asMap().entries.map((entry) {
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
        Expanded(child: _RequestContentWidget(viewModel)),
        // Container(
        //   color: Colors.grey,
        //   height: 30,
        // )
      ],
    );
  }
}

class _RequestContentWidget extends StatelessWidget {
  final RequestViewModel viewModel;

  const _RequestContentWidget(this.viewModel);

  @override
  Widget build(BuildContext context) {
    return IndexedStack(
      index: viewModel.selectIndex(),
      children: [
        Container(
            padding: const EdgeInsets.only(top: 20, left: 20),
            child: SelectableText(
              viewModel.currentReq().name,
              textAlign: TextAlign.center,
              style: const TextStyle(fontSize: fontSize + 5),
            )),
        Center(child: HeaderWidget(viewModel)),
        Container(
          padding: const EdgeInsets.only(left: 5),
          color: Colors.white,
          alignment: Alignment.center,
          child: jsonEditor(viewModel),
        ),
      ],
    );
  }

  JsonEditor jsonEditor(RequestViewModel viewModel) {
    return JsonEditor.string(
      enabled: true,
      jsonString: viewModel.currentReq().reqJson,
      onValueChanged: (value) {
        Log.d("jsonEditor onValueChanged value:$value");
        viewModel.updateRequestJson(value.toString());
      },
    );
  }
}
