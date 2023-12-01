import { final_project_backend } from "../../../declarations/final_project_backend";

const handleProposeSend = async () => {
  const key = Number(proposalCount) + 1; // Generate a unique key for the new proposal.
  if (proposal !== "") { // Proceed only if the proposal text is not empty.
      setLoading(true); // Indicate the start of the process.
      await final_project_backend.create_proposal(key, {
          description: proposal,
          is_active: true,
      }); // Send the proposal to the backend.
      console.log("Proposal Sent"); // Log the successful operation.
      setLoading(false); // Indicate the end of the process.
      setProposal(""); // Reset the proposal text.
      window.location.reload(); // Refresh the page to reflect the new proposal.
  }
};