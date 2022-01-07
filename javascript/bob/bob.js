"use strict";

export const hey = (message) => {
  let msgType = returnMessageType(message);

  switch (msgType) {
    case 'question':
      return 'Sure.';
    case 'yelling':
      return 'Whoa, chill out!';
    case 'yellingQuestion':
      return 'Calm down, I know what I\'m doing!';
    case 'empty':
      return 'Fine. Be that way!';
    case 'other':
      return 'Whatever.';
    default:
      return 'undefined case';
  }
};

function isQuestion(message) {
  return message.endsWith('?');
}

function isYelling(message) {
  return ( message === message.toUpperCase() && message !== message.toLowerCase() );
}

function isYellingQuestion(message) {
  return ( isQuestion(message) && isYelling(message) );
}

function isEmpty(message) {
  return ( message === '' );
}

function sanitizeMessage(message) {
  return message.trim();
}

function returnMessageType(message) {
  message = sanitizeMessage(message);

  const IS_QUESTION = isQuestion(message);
  const IS_YELLING = isYelling(message);
  const IS_YELLING_QUESTION = isYellingQuestion(message);
  const IS_EMPTY = isEmpty(message);

  if (IS_YELLING_QUESTION) {
    return 'yellingQuestion';
  }
  else if (IS_QUESTION) {
    return 'question';
  }
  else if (IS_YELLING) {
    return 'yelling';
  }
  else if (IS_EMPTY) {
    return 'empty';
  }
  else {
    return 'other';
  }
}
