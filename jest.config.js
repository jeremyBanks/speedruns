module.exports = {
  transform: {
    "^.+\\.(ts|tsx)$": "ts-jest",
  },
  snapshotSerializers: ["jest-serializer-html"],
};
