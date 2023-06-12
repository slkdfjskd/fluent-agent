

const navProjectExpandStatus = "nav_project_expand_status_";
const navServiceExpandStatus = "nav_service_expand_status_";
const logExpandStatus = "log_expand_status";
const requestSelectedStatus = "request_selected_status";
const requestTabSelectedStatus = "request_tab_selected_status_";
const responseTabSelectedStatus = "response_tab_selected_status_";

String requestTabSelectedStatusKey(int requestId) {
  return "$requestTabSelectedStatus$requestId";
}

String responseTabSelectedStatusKey(int requestId) {
  return "$responseTabSelectedStatus$requestId";
}

String navProjectExpandStatusKey(int projectId) {
  return "$navProjectExpandStatus$projectId";
}

String navServiceExpandStatusKey(int projectId, String serviceName) {
  return "$navServiceExpandStatus${projectId}_$serviceName";
}