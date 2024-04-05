# Rust MongoDB Auth API Documentation

This API provides endpoints for user registration and login using Actix web framework and MongoDB.

## Endpoints

### Register

Registers a new user with the provided username and password.

- **URL**: `/register`
- **Method**: `POST`
- **Request Body**: JSON

#### Request Body Parameters

| Parameter | Type   | Description                |
|-----------|--------|----------------------------|
| `username`| string | User's username            |
| `password`| string | User's password            |

#### Responses

- **200 OK**: User registered successfully
- **500 Internal Server Error**: Failed to register user

### Login

Authenticates a user with the provided username and password.

- **URL**: `/login`
- **Method**: `POST`
- **Request Body**: JSON

#### Request Body Parameters

| Parameter | Type   | Description                |
|-----------|--------|----------------------------|
| `username`| string | User's username            |
| `password`| string | User's password            |

#### Responses

- **200 OK**: Login successful
- **401 Unauthorized**: Invalid credentials
- **500 Internal Server Error**: Failed to login

## Rust Code Overview

The API is implemented in Rust programming language using Actix web framework and MongoDB driver. Below are the key components of the Rust code:

- **User Struct**: Represents a user with `username` and `password` fields.

- **register Function**: Handles user registration by inserting user data into the MongoDB collection. It takes user data and the MongoDB collection as arguments and returns a HTTP response indicating the registration status.

- **login Function**: Handles user login by verifying user credentials against the MongoDB collection. It takes user data and the MongoDB collection as arguments and returns a HTTP response indicating the login status.

- **main Function**: Initializes the Actix web server, connects to the MongoDB database, selects the 'users' collection, and binds the routes for registration and login.

## Usage

1. Make sure Rust and MongoDB are installed on your system.
2. Run `cargo run` to start the API server.
3. Send POST requests to `/register` and `/login` endpoints using JSON body to register and login users.

## MongoDB Configuration

Update the MongoDB connection URI in the main function with your MongoDB Atlas credentials and cluster information.

```rust
let client = Client::with_uri_str("mongodb+srv://<username>:<password>@<cluster>/<database>")
    .await
    .expect("Failed to initialize MongoDB client");
```

Replace `<username>`, `<password>`, and <cluster> with your MongoDB Atlas credentials and cluster information.



## For Support
For support, send an email to oguzhancart1@gmail.com or you are invited to our telegram channel https://t.me/vsform_tr  
 
## Contact Me
<p align="center">
<a href="https://codepen.io/oguzhan1881" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/codepen.svg" alt="oguzhan1881" height="50" width="50" /></a>
<a href="https://dev.to/oguzhan18" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/devto.svg" alt="oguzhan18" height="50" width="50" /></a>
<a href="https://twitter.com/oguzhancart" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/twitter.svg" alt="oguzhancart" height="50" width="50" /></a>
<a href="https://linkedin.com/in/o%c4%9fuzhan-%c3%a7art-b73405199/" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/linked-in-alt.svg" alt="o%c4%9fuzhan-%c3%a7art-b73405199/" height="50" width="50" /></a>
<a href="https://codesandbox.com/oguzhan18" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/codesandbox.svg" alt="oguzhan18" height="50" width="50" /></a>
<a href="https://instagram.com/oguzhan_cart" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/instagram.svg" alt="oguzhan_cart" height="50" width="50" /></a>
<a href="https://dribbble.com/cart188" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/dribbble.svg" alt="cart188" height="50" width="50" /></a>
<a href="https://hashnode.com/@oguzhancart" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/hashnode.svg" alt="@oguzhancart" height="50" width="50" /></a>
<a href="https://www.leetcode.com/oguzhan18/" target="blank"><img align="center" src="https://raw.githubusercontent.com/rahuldkjain/github-profile-readme-generator/master/src/images/icons/Social/leet-code.svg" alt="oguzhan18/" height="50" width="50" /></a>
</p>
