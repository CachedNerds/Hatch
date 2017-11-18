<h1 align="center">Workflow Standards</h1>

There are many standard practices involving git and github when collaborating on a group project.  The standards defined below are **mandatory**, and everyone working on the project is **required** to adhere to them.

## Git Standards
### Issues
The way that projects are often organized is the high level requirements of the project are broken down into individual tasks called **issues**.  These issues are then assigned to individual collaborators or a collaborator can pick up an issue.

### Branches
The collaborator will then create a branch from the **dev** branch, this allows the collaborator to work on the task and make changes to the project without affecting the **dev** branch.  If the issue you are working on is dependent on a branch that has not yet been merged into **dev**, you will create a branch from **dev**, merge the branch your issue is dependent on into the newly created branch, then implement your issue.  

The **name of the branch must follow the format**, issueN where N is equal to the issue number on github. Example, issue15.  This allows people working on the project to be able to associate your branch with the issue its attempting to implement.

**Collaborators are never permitted to work directly with dev ever.**

**STAY OUT OF MASTER**


### Pull Request
Once a collaborator has completed their task they will push their branch to github and create a **pull request** from their branch to **dev**, please see the naming section below for what to name your **PR**.  Once the pull request has been created, it will require a reviewer to review the pull request.  At this point the reviewer can also leave comments in the pull request, stating changes that may need to be made or inquiring for the reason why the collaborator chose to do something.

If there are comments, address all comments on the same branch. After addressing the comments, update the comment stating that you have addressed the comment. Remember to push the modified code to the remote branch. This should update the original pull-request with the newly modified code.

###### Naming
The name of the **pull request** should be the name of the branch for that
**pull request**.  

```
// Example
The branch I want to merge my changes INTO: dev
The branch that implements issue5: issue5

The name of the PR: issue5
```

**YOU ARE NOT ALLOWED TO REVIEW YOUR OWN PULL REQUEST EVER**

Once the pull request has been approved it is the collaborators responsibility to merge the pull request.
