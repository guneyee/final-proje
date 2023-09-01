# "Final Project" 
 
1. Building a blockchain
2. Simulating a substrate network
3. Adding trusted nodes to a network.
8. Smart contracts


Web3 Auction DApp - README
Welcome to the Web3 Auction DApp project repository! This decentralized application (DApp) leverages blockchain technology to implement an auction platform on the Ethereum network. Participants can place bids on items of interest, and the highest bidder at the end of the auction wins the item.


The Web3 Auction DApp provides a user-friendly interface to participate in Ethereum-based auctions. This project ensures transparency and trust in the auction process through the use of smart contracts. Users can place bids, monitor ongoing auctions, and claim their winnings seamlessly.

Features
Browse ongoing auctions.
Place bids on items of interest.
Automatic outbidding: The system places slightly higher bids on behalf of users when they're outbid.
Real-time updates: Receive instant notifications for outbids and auction end events.
Claiming winnings: Winners of auctions can easily claim their items.
Ethereum Wallet Integration: Connect your Ethereum wallet (e.g., MetaMask) to participate directly.
Getting Started
Follow these steps to set up the project locally and start participating in web3 auctions.

Prerequisites
Node.js: Ensure Node.js is installed. Download it from nodejs.org.
Installation



Clone the repository:
    git clone [https://github.com/p-Final-Case.git](https://github.com/guneyee/-Final-Project)
Install required npm packages:
 npm install
Usage
Start the development server:
 npm start
Open your web browser and navigate to http://localhost:3000 to access the DApp.

Connect your Ethereum wallet (e.g., MetaMask) to the DApp.

Browse ongoing auctions, place bids, and monitor your auction activity.

Smart Contracts
The smart contracts in this project facilitate the auction process. They handle bids, auction creation, and winner determination. These contracts are deployed on the Ethereum blockchain.

AuctionFactory.sol: Responsible for creating new auctions.
Auction.sol: Manages bidding and winner determination for individual auctions.
Testing
Smart contract tests are located in the test folder. These tests ensure the correct functioning of the smart contract. To run the tests, follow these steps:

Open a terminal in the project directory.
Run the following command to execute the tests:
truffle test
This command will initiate the smart contract tests and display the results in the terminal.



Frontend
The DApp frontend is built using modern web technologies including React.js. It provides an intuitive and interactive user interface for auction participation.

React.js: Powers the DApp's user interface.
Web3.js: The Ethereum JavaScript API for smart contract interaction.
MetaMask: A popular Ethereum wallet browser extension for secure transactions.
Contributing
Contributions to this project are welcome! To contribute:

Fork the repository.
Create a new branch for your feature/bug fix.
Make changes and test thoroughly.
Commit with clear and concise messages.
Push changes to your fork.
Submit a pull request describing your changes.
License
This project is licensed under the MIT License.

Thank you for your interest in the Web3 Auction DApp project! For questions or suggestions, reach out to us or open an issue on GitHub. Happy bidding on the blockchain! 🚀
