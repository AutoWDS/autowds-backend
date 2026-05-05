import dayjs from 'dayjs';
import _ from 'lodash';

export function formatTime(time: number) {
  const now = _.now();
  const duration = now - time;
  if (duration < 60 * 1000) {
    return `${Math.floor(duration / 1000)}秒前`;
  }
  if (duration < 60 * 60 * 1000) {
    return `${Math.floor(duration / 60 / 1000)}分钟前`;
  }
  if (duration < 24 * 60 * 60 * 1000) {
    return `${Math.floor(duration / 60 / 60 / 1000)}小时前`;
  }
  if (duration < 7 * 24 * 60 * 60 * 1000) {
    return `${Math.floor(duration / 24 / 60 / 60 / 1000)}天前`;
  }
  return dayjs(time).format('YYYY-MM-DD');
}
