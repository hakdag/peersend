Gets device's public ip address
Parses command
Checks file exists
Calls API if user has the devices (both source and target) provided
Sends file via selected protocol

### Todo
- Add POST /device/validate EP to the API
- EP will require target device name in body
- EP will check user has the source and target devices
- User Id and source device name will be detected from the token
