package com.rudy.ryanto.tiket.exception;

public class OrderException extends RuntimeException{
    private String message;

    public OrderException() {
        super();
    }

    public OrderException(String message) {
        super(message);
    }
}
