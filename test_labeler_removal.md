# Test Plan: Labeler Removal with Tag Cleanup

## Overview
This document outlines the test plan for verifying that removing a labeler from a group also removes their tags from all images in that group.

## Test Scenario

### Setup
1. Create a group with some images
2. Add a labeler to the group
3. Have the labeler tag some images in the group
4. Remove the labeler from the group

### Expected Behavior
1. Labeler should be removed from the group
2. All tags that the labeler applied to images in the group should be removed
3. Tags applied by other labelers should remain intact
4. Tags applied by the removed labeler to images in other groups should remain intact

## Implementation Details

### Backend Changes
- **New Repository Method**: `ImageTagsRepository::delete_by_labeler_and_group()`
  - Gets all images in the specified group
  - Deletes all image tags for the specified labeler from those images
  - Uses efficient batch deletion with `is_in()` filter

- **Updated Service Method**: `AdminService::remove_labeler_from_group()`
  - Calls the new repository method after removing the labeler from the group
  - Includes error handling with warning logs if tag removal fails
  - Continues with success response even if tag removal fails (non-blocking)

### Database Operations
1. Remove labeler from `labeler_groups` table
2. Query all images in the group from `image` table
3. Delete all records from `image_tags` table where:
   - `labeler_id` matches the removed labeler
   - `image_id` is in the list of group images

### Error Handling
- Tag removal is non-blocking - if it fails, a warning is logged but the operation continues
- This ensures that the labeler is still removed from the group even if tag cleanup fails
- The warning helps with debugging and monitoring

## Testing Steps

1. **Start the backend server**
2. **Start the frontend application**
3. **Login as admin**
4. **Create a test group**
5. **Upload some test images to the group**
6. **Create a test labeler**
7. **Add the labeler to the group**
8. **Login as the labeler and tag some images**
9. **Login back as admin**
10. **Remove the labeler from the group**
11. **Verify the labeler is no longer in the group**
12. **Verify the labeler's tags are removed from the images**
13. **Verify other labelers' tags (if any) are still present**

## API Endpoints Used

- `POST /admin/groups/{id}/labelers` - Add labeler to group
- `DELETE /admin/groups/{group_id}/labelers/{labeler_id}` - Remove labeler from group (now includes tag cleanup)
- `GET /admin/groups/{id}` - Get group details (to verify labeler removal)
- `GET /admin/groups/{group_id}/image/{image_id}` - Get image details (to verify tag removal)

## Success Criteria

✅ Labeler is successfully removed from the group
✅ All tags applied by the removed labeler to images in the group are deleted
✅ Tags applied by other labelers remain intact
✅ Tags applied by the removed labeler to images in other groups remain intact
✅ No database inconsistencies or orphaned records
✅ Frontend UI updates correctly to reflect the changes
