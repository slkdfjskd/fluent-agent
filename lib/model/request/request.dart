import '../../api_bridge.dart';
import 'entry.dart';

class RequestVO {
  int id;
  int projectId;
  String name;
  String url;
  ReqType reqType;
  String service;
  String method;
  List<EntryVO> headers;
  List<EntryDTO> params;
  String? reqJson;
  String? respJson;

  int selectIndex = 0;

  List<String> titleBar = [];

  RequestVO(
    this.reqJson,
    this.respJson, {
    required this.id,
    required this.projectId,
    required this.name,
    required this.url,
    required this.reqType,
    required this.service,
    required this.method,
    required this.headers,
    required this.params,
  }) {
    if (reqType == ReqType.GRPC) {
      titleBar = ['Description', 'Metadata', 'Body'];
    } else {
      titleBar = ['Description', 'Headers', 'Body'];
    }
    selectIndex = 2;
  }
}
