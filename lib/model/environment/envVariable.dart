import '../../api_bridge.dart';

class EnvVariableVO {
  EnvVariableDTO dto;

  bool nameEdit;
  bool valueEdit;

  EnvVariableVO(
    this.dto, {
    this.valueEdit = false,
    this.nameEdit = false,
  });
}
