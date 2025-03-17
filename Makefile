up:
	docker compose up -d --build
down:
	docker compose down

start:
	cargo leptos watch

checks: fmt test clippy test

fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

build-front:
	rm -rf bot-static/*
	cd mini-app/sf-view; yarn lint
	cd mini-app/sf-view; yarn build
	cp -rf mini-app/sf-view/dist/* bot-static

deploy-front: build-front
	sh ./scripts/sync.sh

restart-nginx: 
	sudo docker image prune -a -f
	sudo docker compose build nginx
	sudo docker compose up -d --build nginx

restart-back: 
	cargo build --release	
	sudo docker image prune -a -f
	sudo docker compose build backend
	sudo docker compose up -d --build backend

restart: restart-nginx restart-back

logs:
	sudo docker container logs ledger-backend-1	


