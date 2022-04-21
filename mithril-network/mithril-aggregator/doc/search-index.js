var searchIndex = JSON.parse('{\
"mithril_aggregator":{"doc":"","t":[0,0,0,0,3,3,3,3,3,3,3,3,3,3,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,5,5,5,5,5,5,5,5,3,11,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,11,11],"n":["entities","fake_data","http_server","snapshotter","Beacon","Certificate","CertificatePending","Error","ProtocolParameters","Signer","SignerWithStake","SingleSignature","Snapshot","Stake","beacon","block","block","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","certificate_hash","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","code","completed_at","created_at","default","default","default","default","default","default","default","default","default","default","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","digest","digest","epoch","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","hash","index","into","into","into","into","into","into","into","into","into","into","k","locations","m","message","multisignature","ne","ne","ne","ne","ne","ne","ne","ne","ne","ne","network","new","new","new","new","new","new","new","new","new","new","participants","party_id","party_id","party_id","phi_f","previous_hash","previous_hash","protocol_parameters","protocol_parameters","serialize","serialize","serialize","serialize","serialize","serialize","serialize","serialize","serialize","serialize","signature","signers","size","stake","stake","started_at","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","verification_key","verification_key","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","beacon","certificate","certificate_pending","protocol_parameters","signers","signers_with_stakes","single_signatures","snapshots","Server","borrow","borrow_mut","from","into","new","start","try_from","try_into","type_id","vzip","Snapshotter","borrow","borrow_mut","from","into","new","run","try_from","try_into","type_id","vzip"],"q":["mithril_aggregator","","","","mithril_aggregator::entities","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mithril_aggregator::fake_data","","","","","","","","mithril_aggregator::http_server","","","","","","","","","","","mithril_aggregator::snapshotter","","","","","","","","","",""],"d":["","","","","Beacon represents a point in the Cardano chain at which a …","Certificate represents a Mithril certificate embedding a …","CertificatePending represents a pending certificate in the …","Internal error representation","Protocol cryptographic parameters","Signer represents a signing participant in the network","Signer represents a signing party in the network …","SingleSignature represents a single signature originating …","Snapshot represents a snapshot file and its metadata","Stake represents the stakes of a participant in the …","Current Beacon","Cardano chain block number","Cardano chain block number","","","","","","","","","","","","","","","","","","","","","Hash of the associated certificate","","","","","","","","","","","","","","","","","","","","","error code","Date and time at which the certificate was completed (when …","Date and time at which the snapshot was created","","","","","","","","","","","","","","","","","","","","","Digest that is signed by the signer participants","Digest that is signed by the signer participants","Cardano chain epoch number","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Hash of the current certificate","The index of the lottery won that lead to the single …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Quorum parameter","Locations where the binary content of the snapshot can be …","Security parameter (number of lotteries)","error message","STM multisignature created from a quorum of single …","","","","","","","","","","","Cardano network","Beacon factory","CertificatePending factory","Certificate factory","Error factory","ProtocolParameters factory","Signer factory","SignerWithStake factory","SingleSignature factory","Snapshot factory","Stake factory","The list of the participants (potential signers) with …","The unique identifier of the signer","The unique identifier of the signer","The unique identifier of the signer","f in phi(w) = 1 - (1 - f)^w, where w is the stake of a …","Hash of the previous certificate","Hash of the previous certificate","Current Protocol parameters","Protocol parameters","","","","","","","","","","","The single signature of the digest","Current Signers with stakes","Size of the snapshot file in Bytes","","","Date and time at which the certificate was initialized and …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The public key used to authenticate signer signature","The public key used to authenticate signer signature","","","","","","","","","","","Fake Beacon","Fake Certificate","Fake CertificatePending","Fake ProtocolParameters","Fake Signers","Fake SignersWithStake","Fake SingleSignatures","Fake Snapshots","Server","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Server factory","Start","","","","","Snapshotter","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Server factory","Start","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,9,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,4,3,9,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,3,9,2,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,3,8,2,1,3,4,5,6,7,8,9,10,5,9,5,4,3,2,1,3,4,5,6,7,8,9,10,2,2,1,3,4,5,6,7,8,9,10,3,6,7,8,5,1,3,1,3,2,1,3,4,5,6,7,8,9,10,8,1,9,7,10,3,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,2,1,3,4,5,6,7,8,9,10,6,7,2,1,3,4,5,6,7,8,9,10,0,0,0,0,0,0,0,0,0,11,11,11,11,11,11,11,11,11,11,0,12,12,12,12,12,12,12,12,12,12],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],null,[[["",0]],["beacon",3]],[[["",0]],["certificatepending",3]],[[["",0]],["certificate",3]],[[["",0]],["error",3]],[[["",0]],["protocolparameters",3]],[[["",0]],["signer",3]],[[["",0]],["signerwithstake",3]],[[["",0]],["singlesignature",3]],[[["",0]],["snapshot",3]],[[["",0]],["stake",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],null,null,null,[[],["beacon",3]],[[],["certificatepending",3]],[[],["certificate",3]],[[],["error",3]],[[],["protocolparameters",3]],[[],["signer",3]],[[],["signerwithstake",3]],[[],["singlesignature",3]],[[],["snapshot",3]],[[],["stake",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],null,null,null,[[["",0],["beacon",3]],["bool",0]],[[["",0],["certificatepending",3]],["bool",0]],[[["",0],["certificate",3]],["bool",0]],[[["",0],["error",3]],["bool",0]],[[["",0],["protocolparameters",3]],["bool",0]],[[["",0],["signer",3]],["bool",0]],[[["",0],["signerwithstake",3]],["bool",0]],[[["",0],["singlesignature",3]],["bool",0]],[[["",0],["snapshot",3]],["bool",0]],[[["",0],["stake",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,null,[[["",0],["beacon",3]],["bool",0]],[[["",0],["certificatepending",3]],["bool",0]],[[["",0],["certificate",3]],["bool",0]],[[["",0],["error",3]],["bool",0]],[[["",0],["protocolparameters",3]],["bool",0]],[[["",0],["signer",3]],["bool",0]],[[["",0],["signerwithstake",3]],["bool",0]],[[["",0],["singlesignature",3]],["bool",0]],[[["",0],["snapshot",3]],["bool",0]],[[["",0],["stake",3]],["bool",0]],null,[[["string",3],["u64",0],["u64",0]],["beacon",3]],[[["beacon",3],["protocolparameters",3],["string",3],["vec",3,[["signerwithstake",3]]]],["certificatepending",3]],[[["string",3],["string",3],["u64",0],["protocolparameters",3],["string",3],["string",3],["string",3],["vec",3,[["signerwithstake",3]]],["string",3]],["certificate",3]],[[["option",4,[["value",4]]],["string",3]],["error",3]],[[["u64",0],["u64",0],["f32",0]],["protocolparameters",3]],[[["u64",0],["string",3]],["signer",3]],[[["u64",0],["string",3],["u64",0]],["signerwithstake",3]],[[["u64",0],["u64",0],["string",3]],["singlesignature",3]],[[["string",3],["string",3],["u64",0],["string",3],["vec",3,[["string",3]]]],["snapshot",3]],[[["u64",0]],["stake",3]],null,null,null,null,null,null,null,null,null,[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],null,null,null,null,null,null,[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["beacon",3]],[[["string",3]],["certificate",3]],[[],["certificatepending",3]],[[],["protocolparameters",3]],[[["u64",0]],["vec",3,[["signer",3]]]],[[["u64",0]],["vec",3,[["signerwithstake",3]]]],[[["u64",0]],["vec",3,[["singlesignature",3]]]],[[["u64",0]],["vec",3,[["snapshot",3]]]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[["string",3],["u16",0]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[["u32",0],["string",3]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]]],"p":[[3,"CertificatePending"],[3,"Beacon"],[3,"Certificate"],[3,"Error"],[3,"ProtocolParameters"],[3,"Signer"],[3,"SignerWithStake"],[3,"SingleSignature"],[3,"Snapshot"],[3,"Stake"],[3,"Server"],[3,"Snapshotter"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};