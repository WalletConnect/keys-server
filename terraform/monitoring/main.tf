locals {
  opsgenie_notification_channel = "NNOynGwVz"
  notifications = (
    var.environment == "prod" ?
    [{ uid : local.opsgenie_notification_channel }] :
    []
  )

}
