import { final_project_backend } from "../../../../declarations/final_project_backend";

const handleVote = async (voteId) => {
  let userChoice;
  // ... existing setup for userChoice based on voteId
  setVoting(true); // Indicates voting process start
  const reverseIndex = proposalListLength - index; // Adjust index for backend
  await final_project_backend.vote(reverseIndex, userChoice); // Backend call
  setVoting(false); // Indicates voting process end
  window.location.reload(); // Refresh to show updated vote count
};

const handleEndProposal = async () => {
  setEndingProposal(true); // Indicates ending process start
  const reverseIndex = proposalListLength - index; // Adjust index for backend
  await final_project_backend.end_proposal(reverseIndex); // Backend call
  setEndingProposal(false); // Indicates ending process end
  window.location.reload(); // Refresh to update proposal status
};

const editProposal = async () => {
  if (editInput !== "") { // Check for valid input
      await final_project_backend.edit_proposal(proposalCount, {
          description: editInput,
          is_active: proposal[0].is_active,
      }); // Backend call to update proposal
      setEditMode(false); // Exit edit mode
  }
};

