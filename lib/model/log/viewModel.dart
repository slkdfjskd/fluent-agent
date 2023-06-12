
import 'package:expandable/expandable.dart';
import 'package:flutter/material.dart';

import '../../api_bridge.dart';
import '../../common/global.dart';
import '../../common/log.dart';

class LogViewModel with ChangeNotifier {
  final ScrollController _controller = ScrollController();

  final List<RequestLogVO> _logs = [];
  final PageInfo _currentPageInfo = PageInfo();

  bool _nextLoading = false;

  LogViewModel() {
    _controller.addListener(_scrollListener);
  }

  ScrollController get controller => _controller;

  Future<void> loadNextPage() async {
    Log.d("loadNextPage");
    var result = await Global.rsLib.listNextRequestLog(
        lastId: _currentPageInfo.lastId,
        keyword: _currentPageInfo.keyword,
        pageSize: _currentPageInfo.pageSize);
    if (result.code != Code.OK) {
      // TODO 错误提示
      return;
    }
    var data = result.data;
    // 只有第一页获取firstId
    if (_currentPageInfo.lastId == 0) {
      _currentPageInfo.firstId = data!.firstId;
      _jumpToBottom();
    }
    _currentPageInfo.lastId = data!.lastId;
    _currentPageInfo.keyword = data.keyword;
    _currentPageInfo.pageSize = data.pageSize;
    var requestLogs = <RequestLogVO>[];
    for (var element in data.requestLogs) {
      requestLogs.add(RequestLogVO(element));
    }
    _logs.addAll(requestLogs);

    if (requestLogs.isNotEmpty) {
      notifyListeners();
    }
  }

  Future<void> loadPrePage() async {
    Log.d("loadPrePage");
    var result = await Global.rsLib.listPreRequestLog(
        firstId: _currentPageInfo.firstId,
        keyword: _currentPageInfo.keyword,
        pageSize: _currentPageInfo.pageSize);
    if (result.code != Code.OK) {
      // TODO 错误提示
      return;
    }
    var data = result.data;
    _currentPageInfo.firstId = data!.firstId;
    _currentPageInfo.keyword = data.keyword;
    _currentPageInfo.pageSize = data.pageSize;
    var requestLogs = <RequestLogVO>[];
    for (var element in data.requestLogs) {
      requestLogs.add(RequestLogVO(element));
    }
    _logs.insertAll(0, requestLogs);
    notifyListeners();
    await _animateToBottom();
  }

  Future<void> _animateToBottom() async {
    // Log.d(
    //     "offset:${_controller.offset} atEdge:${_controller.position.atEdge} pixels:${_controller.position.pixels} "
    //         "maxScrollExtent: ${_controller.position.maxScrollExtent} minScrollExtent: ${_controller.position.minScrollExtent}");
    if (_controller.position.maxScrollExtent - _controller.offset < 20 && _logs.isNotEmpty) {
      var maxScrollExtent = _controller.position.maxScrollExtent;
      await _controller.animateTo(maxScrollExtent + maxScrollExtent / _logs.length + 1,
          duration: const Duration(milliseconds: 200), curve: Curves.ease);
    }
  }

  Future<void> _jumpToBottom() async {
    Future.delayed(const Duration(milliseconds: 500), () {
      _controller.jumpTo(_controller.position.maxScrollExtent);
    });
  }

  void _scrollListener() {
    // Log.d(
    //     "offset:${_controller.offset} atEdge:${_controller.position.atEdge} pixels:${_controller.position.pixels} "
    //         "maxScrollExtent: ${_controller.position.maxScrollExtent} minScrollExtent: ${_controller.position.minScrollExtent}");
    // 滑动到顶部
    if (_controller.offset < 10) {
      if (!_nextLoading) {
        try {
          _nextLoading = true;
          loadNextPage();
        } finally {
          _nextLoading = false;
        }
      }
    }
  }

  int length() {
    return _logs.length;
  }

  bool isEmpty() {
    return _logs.isEmpty;
  }

  RequestLogVO item(int index) {
    index = _logs.length - 1 - index;
    return _logs[index];
  }

  @override
  void dispose() {
    _controller.dispose();
    for (var log in _logs) {
      log.dispose();
    }
    super.dispose();
  }
}

class PageInfo {
  int lastId = 0;
  int firstId = 0;
  String keyword = "";
  int pageSize = 50;
}

class RequestLogVO {
  final ExpandableController _controller = ExpandableController();

  final RequestLogDTO _requestLog;

  RequestLogVO(this._requestLog);

  ExpandableController get controller => _controller;

  RequestLogDTO get requestLog => _requestLog;

  void dispose() {
    _controller.dispose();
  }

}
