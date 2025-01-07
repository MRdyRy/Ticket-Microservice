package com.rudy.ryanto.tiket.exception;

public class TiketException extends RuntimeException {
    private String message;

    public TiketException() {
        super();
    }

    public TiketException(String message) {
        super(message);
    }
}
