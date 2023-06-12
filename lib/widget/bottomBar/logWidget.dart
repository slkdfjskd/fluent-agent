import 'package:fluent/widget/bottomBar/logList.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../common/colors.dart';
import '../../model/bottomBar/viewModel.dart';

class LogWidget extends StatelessWidget {
  
  const LogWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<BottomBarViewModel>(context);
    return AnimatedContainer(
      decoration: const BoxDecoration(
          color: Colors.white70,
          border: Border(bottom: BorderSide(width: 1, color: rippleColor))),
      width: double.infinity,
      height: viewModel.isOpenWithLog ? 250 : 0,
      duration: const Duration(milliseconds: 200),
      curve: Curves.ease,
      child: const LogListWidget(),
    );
  }
}
