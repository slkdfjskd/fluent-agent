import 'package:flutter/material.dart';

class EmptyResponseWidget extends StatelessWidget {
  const EmptyResponseWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.only(left: 10, top: 10),
      color: Colors.white,
      alignment: Alignment.center,
      child: const Text("No RPC/HTTP Exchange"),
    );
  }
}
