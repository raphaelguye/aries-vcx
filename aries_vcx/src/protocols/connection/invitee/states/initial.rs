use crate::protocols::connection::invitee::states::invited::InvitedState;
use messages::diddoc::aries::diddoc::AriesDidDoc;
use messages::protocols::connection::invite::Invitation;
use messages::protocols::connection::problem_report::ProblemReport;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialState {
    problem_report: Option<ProblemReport>,
    pub did_doc: Option<AriesDidDoc>,
}

impl From<(InitialState, Invitation, AriesDidDoc)> for InvitedState {
    fn from((_state, invitation, did_doc): (InitialState, Invitation, AriesDidDoc)) -> InvitedState {
        trace!("ConnectionInvitee: transit state from InitialState to InvitedState");
        InvitedState { invitation, did_doc }
    }
}

impl InitialState {
    pub fn new(problem_report: Option<ProblemReport>, did_doc: Option<AriesDidDoc>) -> Self {
        InitialState {
            problem_report,
            did_doc,
        }
    }
}
