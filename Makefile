all:
	cd system && bundle install && bundle exec ruby gen.rb ..
