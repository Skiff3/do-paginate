do-paginate Documentation:

Welcome to the documentation for the do-paginate Library! This library provides a flexible way to paginate data in your Rust applications.

    Introduction
    Getting Started
        Installation
        Basic Usage
    Advanced Usage
        Customization
        Examples	
        
  

Introduction

Pagination is a common requirement in web applications, and this Rust library aims to simplify the process of implementing pagination. It provides a set of functions and structs that can be easily integrated into your Rust codebase, making it easier to manage paginated data.

Getting Started

Installation

To get started with the Rust Pagination Library, you can add it as a dependency in your Cargo.toml file:

toml

[dependencies]
paginate-web = "0.1"

Basic Usage

Here's a simple example of how to use the Pagination Library in Rust:

rust

use paginate_web::{Page,Pages};

fn main() {
    // Create a pagination instance
    let pagination: Pages = Pages::new(1000, 20);

    // Get the offset of the current page items based on the page number of the url
    let page = pagination.to_page_number(1)
    let current_page_post_offset = page.unwrap_or_default().begin;

    // Get the total number of pages to be created
    let total_pages = page.unwrap_or_default().length;

    println!("Current Page Offset: {}", current_page_post_offset);
    println!("Items on Page: {:?}", total_pages);
}

Advanced Usage
Customization

The Pagination Library allows for various customization options:

    Specifying the number of items per page.
    Handling events like page changes.


Examples

For more comprehensive usage examples, check out the examples directory in our GitHub repository.

