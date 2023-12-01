import { final_project_backend } from "../../../declarations/final_project_backend";

 // Backend Function Calls
 const handleVote = async (voteId) => {
  let userChoice;
  const handleChoice = () => {
    if (voteId === 1) {
      userChoice = { 'Approve': null }
    } else if (voteId === 2) {
      userChoice = { 'Reject': null }
    } else {
      userChoice = { 'Pass': null }
    }
  }
  handleChoice()
  setVoting(true);
  const vote = await final_project_backend.vote(proposalCount, userChoice);
  setVoting(false);
};

const editProposal = async (count) => {
  editInput !== "" &&
      (await final_project_backend.edit_proposal(proposalCount, {
          description: editInput,
          is_active: proposal[0].is_active,
      }));
  setEditMode(false);
};

await final_project_backend.end_proposal(proposalCount)