Reads token from local storage
Gets user Id from token
Parses command
Gets local ip address - consider changing it to public ip address
Creates device instance
Calls api to create device

### Todo
- Add to API POST /device EP
- EP will require device information in body
- EP return 201 Created if device created successfully
- EP will return a new token which includes the device name
- Overwrite the new token in the local storage


