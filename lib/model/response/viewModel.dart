import 'package:fluent/common/global.dart';
import 'package:fluent/model/response/message.dart';
import 'package:fluent/model/response/response.dart';
import 'package:flutter/cupertino.dart';

import '../../api_bridge.dart';
import '../request/request.dart';

class ResponseViewModel with ChangeNotifier {
  ResponseVO? _currentResp;

  final Map<int, ResponseVO?> _responseMap = {};

  ResponseVO? currentResp() {
    return _currentResp;
  }

  List<String>? tabBar() {
    return _currentResp?.titleBar;
  }

  int? selectIndex() {
    return _currentResp?.selectIndex;
  }

  Future<void> show(RequestVO request) async {
    try {
      var resp = _responseMap[request.id];
      if (resp != null) {
        _currentResp = resp;
        return;
      }
      var result = await Global.rsLib.getLatestRequestLog(requestId: request.id);
      if (result.code == Code.OK) {
        resp = _buildResponse(request, result.data);
        if (resp != null) {
          _responseMap[request.id] = resp;
          _currentResp = resp;
          return;
        }
      }
      _currentResp = null;
    } finally {
      notifyListeners();
    }
  }

  void selectSecondary(int index, bool isRequest) {
    if (isRequest) {
      _currentResp?.request.selectIndex = index;
    } else {
      _currentResp?.response?.selectIndex = index;
    }
    notifyListeners();
  }

  int? selectSecondaryIndex(bool isRequest) {
    if (isRequest) {
      return _currentResp?.request.selectIndex;
    } else {
      return _currentResp?.response?.selectIndex;
    }
  }

  void select(int index) {
    _currentResp?.selectIndex = index;
    notifyListeners();
  }

  void updateResponse(SendRequest request, SendRequestResult result) {
    var resp = ResponseVO(
        code: result.code,
        msg: result.msg,
        url: request.url,
        info: result.info,
        request: MessageVO(
            reqType: request.reqType,
            id: request.requestId,
            headers: request.headers,
            body: request.reqJson));
    if (result.code == Code.OK && result.resp != null) {
      resp.response = MessageVO(
          reqType: request.reqType,
          id: 0,
          headers: result.resp!.headers,
          body: result.resp!.body);
    }
    _responseMap[request.requestId] = resp;
    if (_currentResp == null || _currentResp!.request.id == request.requestId) {
      _currentResp = resp;
      notifyListeners();
    }
  }

  ResponseVO? _buildResponse(RequestVO request, RequestLogDTO? requestLog) {
    if (requestLog == null) {
      return null;
    }
    var err = requestLog.error;
    var code = Code.OK;
    var msg = "";
    if (err != null) {
      code = err.code;
      msg = err.msg;
    }
    var resp = ResponseVO(
        code: code,
        msg: msg,
        url: requestLog.baseUrl,
        info: requestLog.info,
        request: MessageVO(
            reqType: request.reqType,
            id: requestLog.requestId,
            headers: requestLog.request.metadata,
            body: requestLog.request.body));
    if (requestLog.response != null) {
      resp.response = MessageVO(
          reqType: request.reqType,
          id: 0,
          headers: requestLog.response?.metadata,
          body: requestLog.response?.body);
    }
    return resp;
  }
}
