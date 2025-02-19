
# Rust Program for Group Assignment Exercise

This Rust program aims to automatically assign students to groups, each associated with a topic, while ensuring each student fall under a group and has a topic. 
The groups and their details are saved to files, which are only accessible by the program in a read-only format. Additionally, the program ensures an even distribution of students across groups.

## Requirements

- **Student Registration**: Students must be registered within the system.
- **Shuffling**: Students are shuffled randomly to ensure fairness in group assignment.
  
- **Topics**:
  
  - Each group is assigned a unique topic.
  - The number of topics cannot exceed the number of students.
  - Topics form groups
  - 

- **Group Setup**:

  - Each group must have a label.
    - Students should be evenly distributed across the groups based on topic difficulty.
    - A student cannot belong to more than one group.
    - 
- **File Management**:
  
  - All the  groups are saved to a  file, which should be read-only and accessible only by the program.
  
## Key Features

1. **Balanced Group Allocation**:
    - Groups will have an even distribution of students based on the total number of students available.
    - No student will be assigned to more than one group.

2. **Topic Assignment**:
    - Each group will be assigned a unique topic. The number of topics should not exceed the number of students, ensuring that every group has a dedicated topic.


## Objectives

- **Group File Creation**: Every group be in the  unique file with its label and associated students.
- **File Security**: Ensure that the files are stored in a secure manner, where they are read-only and can only be accessed by the program.
- **Balanced Group Distribution**: Distribute students evenly across all groups based on class size.
  
### Optional

- **File Storage in binary**: For more efficient storage and retrieval of group data, the files may be saved in a binary format.
  
## Usage
  Pull the docker image from my github container registry  packages and run in a docker container.
```bash
docker pull ghcr.io/micheal-ndoh/groups:slim
```
You can use 

```bash
docker run -it ghcr.io/micheal-ndoh/groups:slim
```
