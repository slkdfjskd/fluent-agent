import 'package:expandable/expandable.dart';
import 'package:fluent/api_bridge.dart';
import 'package:fluent/widget/navigation/projectPopupMenu.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/navProject/navRequest.dart';
import '../../model/navProject/navService.dart';
import '../../model/navProject/viewModel.dart';
import '../../model/protoFile/viewModel.dart';

class ProjectWidget extends StatelessWidget {
  const ProjectWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Flexible(
        child: Column(
      children: [
        _ProjectTitleBarWidget(),
        _ProjectListWidget(),
      ],
    ));
  }
}

class _ProjectTitleBarWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<NavProjectViewModel>(context);
    return Row(
      children: [
        IconButton(
            splashRadius: 15,
            iconSize: iconSize + 2,
            onPressed: () async {
              Log.d('navProjectList: $viewModel');
              createNewProject(viewModel);
            },
            icon: const Icon(Icons.add)),
        Expanded(
            flex: 5,
            child: Container(
                height: 25,
                padding: const EdgeInsets.only(right: 3),
                child: TextField(
                  controller: viewModel.filterController,
                  autofocus: false,
                  maxLength: 64,
                  style: const TextStyle(
                    fontSize: fontSize,
                  ),
                  decoration: const InputDecoration(
                    counterText: '',
                    contentPadding: EdgeInsets.only(top: 0, bottom: 0),
                    border: OutlineInputBorder(
                        borderSide: BorderSide(width: 0.5),
                        borderRadius: BorderRadius.all(Radius.circular(3))),
                    hintText: 'filter',
                    prefixIcon: Icon(Icons.filter_list, size: iconSize + 2),
                  ),
                ))),
      ],
    );
  }

  Future<void> createNewProject(NavProjectViewModel navProjectViewModel) async {
    navProjectViewModel.createProject(
        name: 'New Project', reqType: ReqType.GRPC);
  }
}

class _ProjectListWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<NavProjectViewModel>(context);
    var protoFileViewModel = Provider.of<ProtoFileViewModel>(context);
    return Flexible(
        child: ListView.builder(
            controller: ScrollController(),
            shrinkWrap: true,
            itemCount: viewModel.length(),
            itemBuilder: (ctx, index) =>
                _ProjectItemWidget(index, viewModel, protoFileViewModel)));
  }
}

class _ProjectItemWidget extends StatelessWidget {
  final NavProjectViewModel _viewModel;

  final ProtoFileViewModel _protoFileViewModel;
  final int _index;

  const _ProjectItemWidget(
      this._index, this._viewModel, this._protoFileViewModel);

  @override
  Widget build(BuildContext context) {
    return ExpandableNotifier(
        controller: _viewModel.getNavProject(_index).expandableController,
        child: ExpandablePanel(
          theme: const ExpandableThemeData(
              iconPadding:
                  EdgeInsets.only(left: 3, right: 0, top: 6, bottom: 6),
              iconRotationAngle: 1.6,
              expandIcon: Icons.navigate_next,
              inkWellBorderRadius: BorderRadius.all(Radius.circular(5)),
              iconPlacement: ExpandablePanelIconPlacement.left,
              iconSize: iconSize),
          header: GestureDetector(
            child: Container(
                color: Colors.transparent,
                height: 28,
                alignment: Alignment.centerLeft,
                child: _EditProjectNameWidget(_viewModel, _index)),
            onSecondaryTapDown: (TapDownDetails details) {
              // details.globalPosition.
              showPopupMenu(
                  context,
                  details.globalPosition.dx,
                  details.globalPosition.dy,
                  _viewModel,
                  _protoFileViewModel,
                  _index);
              Log.d('onSecondaryTap');
            },
          ),
          expanded: _ServiceListWidget(_index, _viewModel),
          collapsed: Container(),
        ));
  }
}

class _EditProjectNameWidget extends StatelessWidget {
  final NavProjectViewModel viewModel;
  final int index;

  const _EditProjectNameWidget(this.viewModel, this.index, {Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    Widget content;
    var projectName = viewModel.getNavProject(index).projectName;
    if (viewModel.isEditProjectNameState(index)) {
      TextEditingController textFieldController =
          TextEditingController.fromValue(TextEditingValue(
              text: projectName,
              selection: TextSelection(
                  baseOffset: 0, extentOffset: projectName.length)));
      var focusNode = FocusNode();
      focusNode.addListener(() {
        Log.d('focusNode Listener: ${focusNode.hasFocus}');
        if (!focusNode.hasFocus) {
          viewModel.setEditProjectNameState(index, false);
          viewModel.updateProjectName(index, textFieldController.value.text);
          textFieldController.dispose();
          focusNode.dispose();
        }
      });
      content = Container(
          height: 25,
          padding: const EdgeInsets.only(right: 3, top: 2),
          child: TextButton.icon(
              onPressed: null,
              icon: const Icon(Icons.folder, color: iconColor, size: iconSize),
              label: TextField(
                  focusNode: focusNode,
                  controller: textFieldController,
                  autofocus: true,
                  maxLength: 64,
                  style: const TextStyle(
                    fontSize: fontSize,
                  ),
                  decoration: const InputDecoration(
                    counterText: '',
                    contentPadding: EdgeInsets.only(left: 5, top: 0, bottom: 0),
                    border: OutlineInputBorder(
                        borderSide: BorderSide(width: 0.5),
                        borderRadius: BorderRadius.all(Radius.circular(3))),
                  ))));
    } else {
      content = GestureDetector(
          onDoubleTap: () {
            Log.d('ProjectName onDoubleTap');
            viewModel.setEditProjectNameState(index, true);
          },
          child: TextButton.icon(
              onPressed: null,
              icon: const Icon(Icons.folder, color: iconColor, size: iconSize),
              label: Text(projectName,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                      color: blackFontColor, fontSize: fontSize))));
    }
    return content;
  }
}

class _ServiceListWidget extends StatelessWidget {
  final NavProjectViewModel _viewModel;
  final int _projectIndex;

  const _ServiceListWidget(this._projectIndex, this._viewModel, {Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var navServiceList = _viewModel.getNavProject(_projectIndex).services;
    return ListView.builder(
        shrinkWrap: true,
        itemCount: navServiceList.length,
        itemBuilder: (ctx, index) => _ServiceItemWidget(index, navServiceList[index], _projectIndex, _viewModel));
  }
}

class _ServiceItemWidget extends StatelessWidget {
  final int _index;
  final NavServiceVO _navService;
  final int _projectIndex;
  final NavProjectViewModel _viewModel;

  const _ServiceItemWidget(this._index, this._navService, this._projectIndex, this._viewModel);

  @override
  Widget build(BuildContext context) {
    return Container(
        margin: const EdgeInsets.only(left: 10),
        child: ExpandableNotifier(
            controller: _navService.expandableController,
            child: ExpandablePanel(
              theme: const ExpandableThemeData(
                  iconPadding:
                      EdgeInsets.only(left: 3, right: 0, top: 6, bottom: 6),
                  iconRotationAngle: 1.6,
                  expandIcon: Icons.navigate_next,
                  inkWellBorderRadius: BorderRadius.all(Radius.circular(5)),
                  iconPlacement: ExpandablePanelIconPlacement.left,
                  iconSize: iconSize),
              header: Container(
                  height: 28,
                  alignment: Alignment.centerLeft,
                  child: TextButton.icon(
                      onPressed: null,
                      icon: const Icon(Icons.dvr,
                          size: iconSize, color: iconColor),
                      label: Text(_navService.name,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                              fontSize: fontSize, color: blackFontColor)))),
              expanded: _RequestListWidget(_projectIndex, _index, _viewModel),
              collapsed: Container(),
            )));
  }
}

class _RequestListWidget extends StatelessWidget {
  final NavProjectViewModel _viewModel;
  final int _projectIndex;
  final int _serviceIndex;

  const _RequestListWidget(this._projectIndex, this._serviceIndex, this._viewModel,
      {Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var navRequestList =
        _viewModel.getNavProject(_projectIndex).services[_serviceIndex].requests;
    return ListView.builder(
        shrinkWrap: true,
        itemCount: navRequestList.length,
        itemBuilder: (ctx, index) => _RequestItemWidget(navRequestList[index], _viewModel));
  }

}

class _RequestItemWidget extends StatelessWidget {
  final NavProjectViewModel _viewModel;
  final NavRequestVO _navRequest;
  const _RequestItemWidget(this._navRequest, this._viewModel);


  @override
  Widget build(BuildContext context) {
    var isSelected =
    _viewModel.isSelectedRequest(_navRequest.id);
    var bgColor = isSelected ? selectedColor : Colors.transparent;
    var textColor = isSelected ? whiteFontColor : blackFontColor;

    return Container(
        decoration: BoxDecoration(
            color: bgColor,
            borderRadius: const BorderRadius.all(Radius.circular(2))),
        child: TextButton.icon(
          style: TextButton.styleFrom(
              alignment: Alignment.centerLeft,
              padding: const EdgeInsets.only(left: 32)),
          icon:
          const Icon(Icons.request_quote, size: iconSize, color: iconColor),
          label: Text(
            _navRequest.name,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(fontSize: fontSize, color: textColor),
          ),
          onPressed: () {
            Log.d("onPressed");
            _viewModel.requestViewModel.show(_navRequest.id);
            _viewModel.setSelectRequestId(_navRequest.id);
          },
        ));
  }

}
