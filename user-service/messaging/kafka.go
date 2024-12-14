package messaging

import (
	"context"
	"encoding/json"
	"log"
	"user-service/domain"

	"github.com/segmentio/kafka-go"
)

type KafkaProducer struct {
	writer *kafka.Writer
}

type AuditMessage struct {
	Data   domain.User
	Event  string
	Status string
}

func NewKafkaProducer(brokerAddr []string, topic string) *KafkaProducer {
	return &KafkaProducer{
		writer: &kafka.Writer{
			Addr:     kafka.TCP(brokerAddr...),
			Topic:    topic,
			Balancer: &kafka.LeastBytes{},
		},
	}
}

func (p *KafkaProducer) SendAuditMessage(msg AuditMessage) error {
	msgByte, err := json.Marshal(msg)
	if err != nil {
		log.Printf("error serialize message: %s", err)
		return err
	}

	err = p.writer.WriteMessages(context.Background(), kafka.Message{
		Value: msgByte,
	})
	if err != nil {
		log.Printf("error sending message: %s", err)
		return err
	}

	log.Printf("success sending message : %s", msg.Data.Email)
	return nil

}
