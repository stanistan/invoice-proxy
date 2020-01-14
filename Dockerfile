FROM php:7.4-cli

# Deets from
# 1. https://stackoverflow.com/questions/39309262/php-on-docker-using-setlocale
# 2. https://stackoverflow.com/questions/14112111/php-money-format-no-comma-delimiter
RUN apt-get update && apt-get install -y locales && apt-get clean
RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
    locale-gen
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8
