import 'package:flutter/material.dart';

enum IndicatorStatus {
  sending,
  successed,
  failed,
  ready,
}

class IndicatorViewModel with ChangeNotifier {
  String status = "Ready";
  String method = "";
  bool indicatorVisible = false;

  updateStatus(IndicatorStatus status, String method, bool indicatorVisible) {
    this.status = _transformIndicatorStatus(status);
    this.method = method;
    this.indicatorVisible = indicatorVisible;

    notifyListeners();
  }

  String _transformIndicatorStatus(IndicatorStatus status) {
    switch (status) {
      case IndicatorStatus.sending:
        return "Sending";
      case IndicatorStatus.successed:
        return "Successed";
      case IndicatorStatus.failed:
        return "Failed";
      case IndicatorStatus.ready:
        return "Ready";
    }
  }
}
