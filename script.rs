function suggestJobs(userProfile) {
    // Simulate AI job suggestion based on user profile
    let jobSuggestions = ["Software Engineer", "Data Scientist", "Product Manager"];
    let userSkills = userProfile.skills;

    // Filter jobs based on user skills (simple example)
    let filteredJobs = jobSuggestions.filter(job => {
        return userSkills.some(skill => job.toLowerCase().includes(skill.toLowerCase()));
    });

    return filteredJobs;
}

// Example user profile
let userProfile = {
    name: "John Doe",
    skills: ["software", "data"]
};

// Get job suggestions for the user
let suggestedJobs = suggestJobs(userProfile);
console.log("Suggested Jobs:", suggestedJobs);
