initSidebarItems({"constant":[["FILE_LIST","List of files that our webserver will serve to the user"]],"fn":[["main",""]],"mod":[["sseq",""]],"struct":[["ResolutionManager","ResolutionManager is a struct that manipulates an AlgebraicObjectsBundle. At the moment, it only understands the \"resolve\" command which causes it to resolve a module and report back the results."],["Server","The server implements the `ws::Handler` trait. It doesn't really do much. When we receive a request, it is either looking for some static files, as specified in `FILE_LIST`, or it is WebSocket message. If it is the former, we return the file. If it is the latter, we parse it into a string and pass it on to ResolutionManager."],["SseqManager",""]]});