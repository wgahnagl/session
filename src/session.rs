pub fn new_session(login_response: xmlrpc::Value) {
    if login_response.get("reason") != None {
        return; //create an error type. failed to create a session
    }

    login_response.get()
}
