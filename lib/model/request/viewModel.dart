import 'package:fluent/common/config/config.dart';
import 'package:fluent/common/global.dart';
import 'package:fluent/common/log.dart';
import 'package:fluent/model/request/request.dart';
import 'package:fluent/model/response/viewModel.dart';
import 'package:flutter/material.dart';
import 'package:fluent/common/config/configKey.dart' as configKey;

import '../../api_bridge.dart';
import '../environment/viewModel.dart';
import '../indicator/indicator.dart';
import '../log/viewModel.dart';
import 'entry.dart';

class RequestViewModel with ChangeNotifier {
  RequestVO _currentReq = _emptyRequest;
  late final ResponseViewModel _responseViewModel;
  late final IndicatorViewModel _indicatorViewModel;
  final EnvironmentViewModel _environmentViewModel;
  final LogViewModel _logViewModel;
  final Map<int, RequestVO?> _requestMap = {};

  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _valueController = TextEditingController();
  final FocusNode _nameFocusNode = FocusNode();
  final FocusNode _valueFocusNode = FocusNode();

  RequestViewModel(this._environmentViewModel, this._logViewModel) {
    _nameFocusNode.addListener(() {
      if (!nameFocusNode.hasFocus) {
        var name = nameController.value.text;
        var value = valueController.value.text;
        if (name.isNotEmpty) {
          addHeader(name, value);
          nameController.text = '';
          valueController.text = '';
        }
      }
    });

    valueFocusNode.addListener(() {
      if (!valueFocusNode.hasFocus) {
        var name = nameController.value.text;
        var value = valueController.value.text;
        if (name.isNotEmpty) {
          addHeader(name, value);
          nameController.text = '';
          valueController.text = '';
        }
      }
    });
  }

  TextEditingController get valueController => _valueController;

  TextEditingController get nameController => _nameController;

  FocusNode get nameFocusNode => _nameFocusNode;

  FocusNode get valueFocusNode => _valueFocusNode;

  set responseViewModel(ResponseViewModel viewModel) {
    _responseViewModel = viewModel;
  }

  set indicatorViewModel(IndicatorViewModel viewModel) {
    _indicatorViewModel = viewModel;
  }

  Future<void> show(int requestId) async {
    var req = _requestMap[requestId];
    req ??= await loadRequest(requestId);
    _currentReq = req!;
    var status = await getInt(configKey.requestTabSelectedStatusKey(_currentReq.id), 2);
    _currentReq.selectIndex = status;
    _responseViewModel.show(req);
    notifyListeners();
  }

  void deleteCache(List<int> requestIds) {
    for (var i = 0; i < requestIds.length && _requestMap.isNotEmpty; i++) {
      _requestMap.remove(requestIds[i]);
    }
    Future.delayed(const Duration(seconds: 0), () async {
      if (_currentReq.id != 0) {
        await show(_currentReq.id);
      }
    });
  }

  void select(int index) {
    _currentReq.selectIndex = index;
    putInt(configKey.requestTabSelectedStatusKey(_currentReq.id), index);
    notifyListeners();
  }

  List<String> tabBar() {
    return _currentReq.titleBar;
  }

  int selectIndex() {
    return _currentReq.selectIndex;
  }

  RequestVO currentReq() {
    return _currentReq;
  }

  setHeaderSelect(int index, bool selected) {
    _currentReq.headers[index].selected = selected;
    notifyListeners();
  }

  addHeader(String name, String value) {
    if (_currentReq == _emptyRequest) {
      return;
    }
    var nameEdit = name.isEmpty;
    var valueEdit = value.isEmpty;
    _currentReq.headers.add(EntryVO(
        name: name,
        value: value,
        selected: true,
        nameEdit: nameEdit,
        valueEdit: valueEdit));
    notifyListeners();
  }

  updateHeader(int index, {String? name, String? value}) {
    var header = _currentReq.headers[index];
    if (name != null) {
      header.name = name;
    }
    if (value != null) {
      header.value = value;
    }
    notifyListeners();
  }

  editHeaderName(int index, bool edit) {
    var header = _currentReq.headers[index];
    header.nameEdit = edit;
    notifyListeners();
  }

  editHeaderValue(int index, bool edit) {
    var header = _currentReq.headers[index];
    header.valueEdit = edit;
    notifyListeners();
  }

  deleteHeader(int index) async {
    _currentReq.headers.removeAt(index);
    await updateRequest(_currentReq);
    notifyListeners();
  }

  Future<void> updateRequestJson(String reqJson) async {
    var request = _currentReq;
    request.reqJson = reqJson;
    await updateRequest(request);
  }

  Future<void> updateRequestUrl(String url) async {
    var request = _currentReq;
    request.url = url;
    await updateRequest(request);
  }

  Future<void> updateRequest(RequestVO req) async {
    var param = UpdateRequest(
        id: req.id,
        name: req.name,
        url: req.url,
        method: req.method,
        headers: transformToEntryDTO(req.headers),
        params: req.params,
        reqJson: req.reqJson == null ? "" : req.reqJson!,
        respJson: req.respJson == null ? "" : req.respJson!);
    var result = await Global.rsLib.updateRequest(request: param);
    if (result.code != Code.OK) {
      // TODO 提示错误
    }
  }

  Future<RequestVO?> loadRequest(int requestId) async {
    RequestVO? req;
    var result = await Global.rsLib.getRequest(requestId: requestId);
    // 获取request 并且放入map
    if (result.code == Code.OK) {
      var r = result.data;
      if (r != null) {
        req = RequestVO(r.reqJson, r.respJson,
            id: r.id,
            projectId: r.projectId,
            name: r.name,
            url: r.url,
            reqType: r.reqType,
            service: r.service,
            method: r.method,
            headers: transformToEntryVO(r.headers),
            params: r.params);
        _requestMap[r.id] = req;
      } else {
        Log.d("request not found requestId:$requestId");
      }
    } else {
      // TODO 报错
    }
    return req;
  }

  Future<void> sendRequest() async {
    var req = _currentReq;
    _indicatorViewModel.updateStatus(IndicatorStatus.sending, req.method, true);
    var sendRequest = SendRequest(
        requestId: req.id,
        url: _environmentViewModel.parseUrl(req.url),
        headers: transformToEntryDTO(req.headers),
        params: req.params,
        reqJson: req.reqJson == null ? "{}" : req.reqJson!,
        envName: _environmentViewModel.envName(),
        reqType: req.reqType);
    var result = await Global.rsLib.sendRequest(param: sendRequest);
    _responseViewModel.updateResponse(sendRequest, result);
    if (result.code == Code.OK) {
      _indicatorViewModel.updateStatus(
          IndicatorStatus.successed, req.method, false);
    } else {
      _indicatorViewModel.updateStatus(
          IndicatorStatus.failed, req.method, false);
    }
    _logViewModel.loadPrePage();
  }

  @override
  void dispose() {
    _nameController.dispose();
    _valueController.dispose();
    super.dispose();
  }
}

List<EntryVO> transformToEntryVO(List<EntryDTO> headers) {
  var result = <EntryVO>[];
  for (var h in headers) {
    result.add(EntryVO(
        name: h.name,
        value: h.value,
        selected: true,
        nameEdit: false,
        valueEdit: false));
  }
  return result;
}

List<EntryDTO> transformToEntryDTO(List<EntryVO> headers) {
  var result = <EntryDTO>[];
  for (var h in headers) {
    if (h.selected) {
      result.add(EntryDTO(name: h.name, value: h.value));
    }
  }
  return result;
}

var _emptyRequest = RequestVO('', '',
    id: 0,
    projectId: 0,
    name: '',
    url: '',
    reqType: ReqType.HTTP,
    service: '',
    method: '',
    headers: [],
    params: []);
