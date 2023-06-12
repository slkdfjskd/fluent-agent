import 'package:fluent/widget/navigation/project.dart';
import 'package:flutter/material.dart';

import '../environment/environments.dart';

class NavigationWidget extends StatelessWidget {
  const NavigationWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const Column(
      children: [
        EnvironmentsWidget(),
        ProjectWidget(),
      ],
    );
  }
}
