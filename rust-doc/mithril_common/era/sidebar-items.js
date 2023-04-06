window.SIDEBAR_ITEMS = {"enum":[["EraReaderError","Error type when [EraReader] fails to return a [EraEpochToken]."],["SupportedEra","The era that the software is running or will run"]],"mod":[["adapters","Module dedicated to EraReaderAdapter implementations."]],"struct":[["EraChecker","EraChecker allows the verification of the current era"],["EraEpochToken","This is a response from the [EraReader]. It contains [EraMarker]s read from the adapter. It can try to cast the given markers to [SupportedEra]s."],["EraMarker","Value object that represents a tag of Era change."],["EraReader","The EraReader is responsible of giving the current Era and the Era to come. It uses an [EraReaderAdapter] to read data from a backend."],["SupportedEraIter","An iterator over the variants of [Self]"],["UnsupportedEraError","Error related to [SupportedEra] String parsing implementation."]],"trait":[["EraReaderAdapter","Adapters are responsible of technically reading the information of [EraMarker]s from a backend."]]};