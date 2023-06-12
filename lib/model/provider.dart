import 'package:fluent/model/bottomBar/viewModel.dart';
import 'package:fluent/model/indicator/indicator.dart';
import 'package:provider/provider.dart';
import 'package:provider/single_child_widget.dart';

import 'environment/environmentList.dart';
import 'environment/viewModel.dart';
import 'log/viewModel.dart';
import 'initData/initData.dart';
import 'navProject/viewModel.dart';
import 'protoFile/viewModel.dart';
import 'request/viewModel.dart';
import 'response/viewModel.dart';

List<SingleChildWidget> providers() {
  var environmentViewModel = EnvironmentViewModel();
  var logViewModel = LogViewModel();
  var requestViewModel = RequestViewModel(environmentViewModel, logViewModel);
  var responseViewModel = ResponseViewModel();
  var indicatorViewModel = IndicatorViewModel();
  requestViewModel.responseViewModel = responseViewModel;
  requestViewModel.indicatorViewModel = indicatorViewModel;

  var bottomBarViewModel = BottomBarViewModel();

  return [
    FutureProvider<Initializer>(
      initialData: Initializer(),
      create: (ctx) async {
        var initializer = Initializer();
        await initializer.init();
        return initializer;
      },
    ),
    ChangeNotifierProxyProvider<Initializer, NavProjectViewModel>(
        create: (ctx) => NavProjectViewModel(requestViewModel),
        update: (ctx, initializer, navProjectViewModel) {
          navProjectViewModel?.initData(initializer.initData);
          return navProjectViewModel!;
        }),
    ChangeNotifierProxyProvider<NavProjectViewModel, ProtoFileViewModel>(
        create: (ctx) => ProtoFileViewModel(),
        update: (ctx, navProjectViewModel, protoFilesViewModel) {
          protoFilesViewModel?.setNavProjectViewModel(navProjectViewModel);
          return protoFilesViewModel!;
        }),
    ChangeNotifierProvider<RequestViewModel>(create: (ctx) => requestViewModel),
    ChangeNotifierProvider<ResponseViewModel>(
        create: (ctx) => responseViewModel),
    ChangeNotifierProvider<IndicatorViewModel>(
        create: (ctx) => indicatorViewModel),
    ChangeNotifierProvider<LogViewModel>(create: (ctx) => logViewModel),
    ChangeNotifierProxyProvider<Initializer, BottomBarViewModel>(
      create: (ctx) => bottomBarViewModel,
      update: (ctx, initializer, bottomBarViewModel) {
        bottomBarViewModel?.initData = initializer.initData;
        return bottomBarViewModel!;
      },
    ),
    FutureProvider<EnvironmentList>(
        initialData: EnvironmentList(),
        create: (ctx) async {
          var list = EnvironmentList();
          await list.load();
          return list;
        }),
    ChangeNotifierProxyProvider<EnvironmentList, EnvironmentViewModel>(
        create: (ctx) => environmentViewModel,
        update: (ctx, list, environmentViewModel) {
          environmentViewModel?.setEnvironmentList(list.groupList);
          return environmentViewModel!;
        }),
  ];
}
