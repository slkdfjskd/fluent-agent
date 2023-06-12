import 'package:fluent/widget/response/emptyResonse.dart';
import 'package:fluent/widget/response/primaryTitleBar.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../model/response/viewModel.dart';

class ResponseWidget extends StatefulWidget {
  const ResponseWidget({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => ResponseState();
}

class ResponseState extends State<ResponseWidget> {
  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<ResponseViewModel>(context);
    if (viewModel.currentResp() == null) {
      return const EmptyResponseWidget();
    } else {
      return PrimaryTitleBarWidget(viewModel: viewModel);
    }
  }
}
