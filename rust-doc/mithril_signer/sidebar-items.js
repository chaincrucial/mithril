window.SIDEBAR_ITEMS = {"enum":[["CertificateHandlerError","Error structure for the Certificate Handler."],["MithrilProtocolInitializerBuilderError","MithrilProtocolInitializerBuilder error structure."],["RuntimeError","This type represents the errors thrown from the Runner."],["SignerState","Different possible states of the state machine."],["SingleSignerError","SingleSigner error structure."]],"struct":[["CertificateHandlerHTTPClient","CertificateHandlerHTTPClient is a http client for an aggregator"],["Config","Client configuration"],["DumbCertificateHandler","This certificate handler is intended to be used by test services. It actually does not communicate with an aggregator host but mimics this behavior. It is driven by a Tester that controls the CertificatePending it can return and it can return its internal state for testing."],["MithrilProtocolInitializerBuilder","This is responsible of creating new instances of ProtocolInitializer."],["MithrilSingleSigner","Implementation of the SingleSigner."],["ProductionServiceBuilder","Create a SignerService instance for Production environment."],["ProtocolInitializerStore","Implementation of the ProtocolInitializerStorer"],["RegisteredState","Structure to hold `Registered` state information."],["SignedState","Structure to hold `Signed` state information."],["SignerRunner","Controller methods for the Signer’s state machine."],["SignerServices","This structure groups all the services required by the state machine."],["StateMachine","The state machine is responsible of the execution of the signer automate."]],"trait":[["CertificateHandler","Trait for mocking and testing a `CertificateHandler`"],["ProtocolInitializerStorer","Store the ProtocolInitializer used for each Epoch. This is useful because protocol parameters and stake distribution change over time."],["Runner","This trait is mainly intended for mocking."],["ServiceBuilder","The ServiceBuilder is intended to manage Services instance creation. The goal of this is to put all this code out of the way of business code."],["SingleSigner","The SingleSigner is the structure responsible of issuing SingleSignatures."]]};