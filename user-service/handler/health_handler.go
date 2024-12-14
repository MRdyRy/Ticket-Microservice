package handler

import "net/http"

func (h *UserHandler) Healthcheck(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Pong, App running healthty"))
}
