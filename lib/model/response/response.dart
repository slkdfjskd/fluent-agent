import '../../api_bridge.dart';
import 'message.dart';

class ResponseVO {
  final Code code;
  final String? msg;
  final String url;
  final SendRequestInfo info;
  final MessageVO request;
  MessageVO? response;

  int selectIndex = 0;

  List<String> titleBar = [];

  ResponseVO({
    required this.code,
    required this.msg,
    required this.url,
    required this.info,
    required this.request,
  }) {
    if (code == Code.OK) {
      titleBar = ['Info', 'Request', 'Response'];
    } else {
      titleBar = ['Info', 'Request', 'Error'];
    }
    selectIndex = 2;
  }
}
