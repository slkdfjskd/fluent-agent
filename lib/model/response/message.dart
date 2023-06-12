import '../../api_bridge.dart';

class MessageVO {
  final int id;
  final List<EntryDTO>? headers;
  final String? body;

  int _selectIndex = 1;

  List<String> titleBar = ['Headers', 'Body'];

  MessageVO({
    required ReqType reqType,
    required this.id,
    required this.headers,
    required this.body,
  }) {
    if (reqType == ReqType.GRPC) {
      titleBar = ['Metadata', 'Body'];
    }
  }

  int get selectIndex => _selectIndex;

  set selectIndex(int value) {
    _selectIndex = value;
  }
}
