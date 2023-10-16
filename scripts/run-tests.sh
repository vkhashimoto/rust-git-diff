#!/bin/sh

if [ -d "scripts/.ssh" ]
then
	rm -r scripts/.ssh
fi

mkdir scripts/.ssh

ssh-keygen -t ed25519 -C "app@docker" -f scripts/.ssh/id_rsa -N ''


docker compose -f scripts/docker-compose.yml up --build --exit-code-from app_test
